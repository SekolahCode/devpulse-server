use axum::{extract::{Path, State}, Json};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{ai::{select_model, AnalysisContext, ApiKeys, Model}, errors::AppError, AppState};

/// GET /api/issues/:id/analyze — return cached analysis without calling an AI provider.
pub async fn get_analysis(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let cached = sqlx::query!(
        r#"
        SELECT root_cause, explanation, fix_suggestion, code_example,
               severity, prevention, model, model_auto, model_reason, analyzed_at
        FROM ai_analyses
        WHERE issue_id = $1
        "#,
        id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("No analysis for this issue yet".into()))?;

    Ok(Json(json!({
        "cached":         true,
        "root_cause":     cached.root_cause,
        "explanation":    cached.explanation,
        "fix_suggestion": cached.fix_suggestion,
        "code_example":   cached.code_example,
        "severity":       cached.severity,
        "prevention":     cached.prevention,
        "model":          cached.model,
        "model_auto":     cached.model_auto,
        "model_reason":   cached.model_reason,
        "analyzed_at":    cached.analyzed_at,
    })))
}

/// POST /api/issues/:id/analyze — run (or re-run) AI analysis, cache result.
///
/// Optional JSON body:
/// ```json
/// { "model": "auto" | "haiku" | "sonnet" | "opus" | "gpt-4o-mini" | "gpt-4o" | "gemini-flash" | "gemini-pro" }
/// ```
/// When omitted or `"auto"`, the model is selected automatically based on
/// issue complexity signals, using the best available provider.
pub async fn analyze_issue(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
    body:         Option<Json<Value>>,
) -> Result<Json<Value>, AppError> {
    let keys = ApiKeys {
        anthropic: state.anthropic_key.as_deref(),
        openai:    state.openai_key.as_deref(),
        gemini:    state.gemini_key.as_deref(),
    };

    if !keys.any() {
        return Err(AppError::BadRequest(
            "No AI provider keys configured. Set ANTHROPIC_API_KEY, OPENAI_API_KEY, or GEMINI_API_KEY.".into()
        ));
    }

    let model_request = body.as_ref()
        .and_then(|b| b.get("model"))
        .and_then(|m| m.as_str())
        .unwrap_or("auto")
        .to_string();

    // Rate limit: prevent re-analysis within 60 seconds
    let too_recent = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM ai_analyses WHERE issue_id = $1 AND analyzed_at > NOW() - INTERVAL '60 seconds')",
        id
    )
    .fetch_one(&state.pg_pool)
    .await?
    .unwrap_or(false);

    if too_recent {
        return Err(AppError::TooManyRequests);
    }

    // Fetch the issue
    let issue = sqlx::query!(
        r#"
        SELECT i.id, i.title, i.level, i.event_count, p.platform
        FROM issues i
        JOIN projects p ON p.id = i.project_id
        WHERE i.id = $1
        "#,
        id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Issue not found".into()))?;

    // Pull the latest event for stack trace + context
    let event = sqlx::query!(
        r#"
        SELECT payload, context
        FROM events
        WHERE issue_id = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        id
    )
    .fetch_optional(&state.pg_pool)
    .await?;

    // ── Build stack trace from payload ────────────────────────────────────────
    let (stacktrace, frame_count, has_exception_chain) = if let Some(ref e) = event {
        let payload: &Value = &e.payload;

        let primary_frames = payload
            .get("exception")
            .and_then(|ex| ex.get("stacktrace"))
            .and_then(|st| st.as_array());

        let has_chain = payload
            .get("exception_chain")
            .and_then(|c| c.as_array())
            .map(|a| !a.is_empty())
            .unwrap_or(false);

        if let Some(frames) = primary_frames {
            let lines: Vec<String> = frames.iter().map(|f| {
                let func = f["function"].as_str().unwrap_or("?");
                let file = f["file"].as_str().unwrap_or("");
                let line = f["line"].as_i64().map(|l| format!(":{l}")).unwrap_or_default();
                format!("  at {func} ({file}{line})")
            }).collect();
            let count = lines.len();
            (lines.join("\n"), count, has_chain)
        } else {
            (issue.title.clone(), 0, has_chain)
        }
    } else {
        (issue.title.clone(), 0, false)
    };

    let context_str = event.as_ref().and_then(|e| {
        e.context.as_ref().map(|c| serde_json::to_string_pretty(c).unwrap_or_default())
    });

    let is_vitals = issue.title == "Performance vitals";
    let platform  = issue.platform.as_deref().unwrap_or("unknown");

    // ── Model selection ───────────────────────────────────────────────────────
    let (model, model_auto, model_reason_str): (Model, bool, Option<String>) =
        if model_request == "auto" || model_request.is_empty() {
            let ctx = AnalysisContext {
                stacktrace_frames:   frame_count,
                level:               issue.level.clone().unwrap_or_default(),
                event_count:         issue.event_count.unwrap_or(0) as i64,
                has_exception_chain,
                is_vitals,
            };
            let (m, reason) = select_model(&ctx, &keys);
            (m, true, Some(reason.to_string()))
        } else {
            let m = Model::from_str(&model_request).unwrap_or(Model::ClaudeSonnet);
            (m, false, None)
        };

    tracing::info!(
        "🤖 Analysing issue {id} with {} (auto={model_auto})",
        model.display_name()
    );

    // ── Call AI provider ──────────────────────────────────────────────────────
    let analysis = crate::ai::analyse_issue(
        &state.http_client,
        &keys,
        &issue.title,
        &stacktrace,
        platform,
        context_str.as_deref(),
        &model,
        model_auto,
        model_reason_str.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("AI analysis failed for issue {id}: {e}");
        AppError::InternalError(format!("AI analysis failed: {e}"))
    })?;

    // ── Upsert into cache ─────────────────────────────────────────────────────
    sqlx::query!(
        r#"
        INSERT INTO ai_analyses
            (issue_id, root_cause, explanation, fix_suggestion, code_example,
             severity, prevention, model, model_auto, model_reason)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (issue_id) DO UPDATE SET
            root_cause     = EXCLUDED.root_cause,
            explanation    = EXCLUDED.explanation,
            fix_suggestion = EXCLUDED.fix_suggestion,
            code_example   = EXCLUDED.code_example,
            severity       = EXCLUDED.severity,
            prevention     = EXCLUDED.prevention,
            model          = EXCLUDED.model,
            model_auto     = EXCLUDED.model_auto,
            model_reason   = EXCLUDED.model_reason,
            analyzed_at    = NOW()
        "#,
        id,
        analysis.root_cause,
        analysis.explanation,
        analysis.fix_suggestion,
        analysis.code_example,
        analysis.severity,
        analysis.prevention,
        analysis.model,
        analysis.model_auto,
        analysis.model_reason,
    )
    .execute(&state.pg_pool)
    .await?;

    tracing::info!("✅ AI analysis cached for issue {id} ({})", analysis.model);

    Ok(Json(json!({
        "cached":         false,
        "root_cause":     analysis.root_cause,
        "explanation":    analysis.explanation,
        "fix_suggestion": analysis.fix_suggestion,
        "code_example":   analysis.code_example,
        "severity":       analysis.severity,
        "prevention":     analysis.prevention,
        "model":          analysis.model,
        "model_auto":     analysis.model_auto,
        "model_reason":   analysis.model_reason,
        "analyzed_at":    chrono::Utc::now(),
    })))
}
