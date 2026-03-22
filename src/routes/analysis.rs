use axum::{extract::{Path, State}, Json};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{errors::AppError, AppState};

/// GET /api/issues/:id/analyze — return cached analysis without calling Claude.
/// Returns 404 if no analysis has been run yet.
pub async fn get_analysis(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let cached = sqlx::query!(
        r#"
        SELECT root_cause, explanation, fix_suggestion, code_example,
               severity, prevention, model, analyzed_at
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
        "analyzed_at":    cached.analyzed_at,
    })))
}

/// POST /api/issues/:id/analyze — run (or re-run) AI analysis, cache result.
/// Rate-limited: returns 429 if called within 60 seconds of last analysis.
pub async fn analyze_issue(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let api_key = state.anthropic_key.clone().ok_or_else(|| {
        AppError::BadRequest("ANTHROPIC_API_KEY is not configured on this server".into())
    })?;

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
        SELECT i.id, i.title, i.level, p.platform
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

    // Build stack trace string from payload
    let stacktrace = event.as_ref().and_then(|e| {
        let payload: &serde_json::Value = &e.payload;
        let frames = payload
            .get("exception")
            .and_then(|ex| ex.get("stacktrace"))
            .and_then(|st| st.as_array())?;

        let lines: Vec<String> = frames.iter().map(|f| {
            let func = f["function"].as_str().unwrap_or("?");
            let file = f["file"].as_str().unwrap_or("");
            let line = f["line"].as_i64().map(|l| format!(":{l}")).unwrap_or_default();
            format!("  at {func} ({file}{line})")
        }).collect();
        Some(lines.join("\n"))
    }).unwrap_or_else(|| issue.title.clone());

    let context_str = event.as_ref().and_then(|e| {
        e.context.as_ref().map(|c| serde_json::to_string_pretty(c).unwrap_or_default())
    });

    let platform = issue.platform.as_deref().unwrap_or("unknown");

    // Call Claude
    let analysis = crate::ai::analyse_issue(
        &state.http_client,
        &api_key,
        &issue.title,
        &stacktrace,
        platform,
        context_str.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("AI analysis failed for issue {id}: {e}");
        AppError::InternalError(format!("AI analysis failed: {e}"))
    })?;

    // Upsert into cache
    sqlx::query!(
        r#"
        INSERT INTO ai_analyses
            (issue_id, root_cause, explanation, fix_suggestion, code_example, severity, prevention, model)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (issue_id) DO UPDATE SET
            root_cause     = EXCLUDED.root_cause,
            explanation    = EXCLUDED.explanation,
            fix_suggestion = EXCLUDED.fix_suggestion,
            code_example   = EXCLUDED.code_example,
            severity       = EXCLUDED.severity,
            prevention     = EXCLUDED.prevention,
            model          = EXCLUDED.model,
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
    )
    .execute(&state.pg_pool)
    .await?;

    tracing::info!("✅ AI analysis cached for issue {id}");

    Ok(Json(json!({
        "cached":         false,
        "root_cause":     analysis.root_cause,
        "explanation":    analysis.explanation,
        "fix_suggestion": analysis.fix_suggestion,
        "code_example":   analysis.code_example,
        "severity":       analysis.severity,
        "prevention":     analysis.prevention,
        "model":          analysis.model,
        "analyzed_at":    chrono::Utc::now(),
    })))
}
