/// Claude API client for AI-powered issue analysis.
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const ANTHROPIC_API: &str = "https://api.anthropic.com/v1/messages";

// ── Model registry ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    Haiku,   // Fast, cheap — simple/common errors, vitals, high-frequency issues
    Sonnet,  // Balanced — standard production errors (default)
    Opus,    // Most capable — deep stacks, exception chains, critical issues
}

impl Model {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "haiku"  | "claude-haiku-4-5"            => Some(Self::Haiku),
            "sonnet" | "claude-sonnet-4-6"            => Some(Self::Sonnet),
            "opus"   | "claude-opus-4-6"              => Some(Self::Opus),
            _ => None,
        }
    }

    pub fn api_id(&self) -> &'static str {
        match self {
            Self::Haiku  => "claude-haiku-4-5-20251001",
            Self::Sonnet => "claude-sonnet-4-6",
            Self::Opus   => "claude-opus-4-6",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Haiku  => "Claude Haiku 4.5",
            Self::Sonnet => "Claude Sonnet 4.6",
            Self::Opus   => "Claude Opus 4.6",
        }
    }

    pub fn max_tokens(&self) -> u32 {
        match self {
            Self::Haiku  => 512,
            Self::Sonnet => 1024,
            Self::Opus   => 2048,
        }
    }
}

// ── Auto-selection logic ──────────────────────────────────────────────────────

pub struct AnalysisContext {
    pub stacktrace_frames:   usize,
    pub level:               String,
    pub event_count:         i64,
    pub has_exception_chain: bool,
    pub is_vitals:           bool,
}

/// Pick the most appropriate model based on issue complexity signals.
/// Returns (Model, reason_string).
pub fn select_model(ctx: &AnalysisContext) -> (Model, &'static str) {
    // Vitals events are pure metrics — arithmetic, not reasoning
    if ctx.is_vitals {
        return (Model::Haiku, "Performance vitals — metrics interpretation doesn't need deep reasoning");
    }

    // Deep complexity signals → Opus
    if ctx.has_exception_chain {
        return (Model::Opus, "Chained exception detected — multi-layer cause analysis requires the most capable model");
    }
    if ctx.stacktrace_frames > 30 {
        return (Model::Opus, "Large stack trace — deep call-graph analysis benefits from extended reasoning");
    }
    if ctx.level == "error" && ctx.stacktrace_frames > 20 {
        return (Model::Opus, "Critical error with complex stack — thorough root-cause reasoning selected");
    }

    // Simple signals → Haiku
    if ctx.is_vitals || ctx.level == "info" {
        return (Model::Haiku, "Info-level event — lightweight analysis is sufficient");
    }
    if ctx.stacktrace_frames < 5 {
        return (Model::Haiku, "Short stack trace — fast model is sufficient for simple errors");
    }
    if ctx.event_count > 500 {
        return (Model::Haiku, "High-frequency issue — well-known error pattern, fast analysis appropriate");
    }

    // Default: balanced Sonnet for standard production errors
    (Model::Sonnet, "Standard production error — balanced model selected for accuracy and speed")
}

// ── Analysis result ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub root_cause:      String,
    pub explanation:     String,
    pub fix_suggestion:  String,
    pub code_example:    Option<String>,
    pub severity:        String,
    pub prevention:      Option<String>,
    pub model:           String,
    pub model_auto:      bool,
    pub model_reason:    Option<String>,
}

// ── API call ──────────────────────────────────────────────────────────────────

pub async fn analyse_issue(
    client:     &Client,
    api_key:    &str,
    title:      &str,
    stacktrace: &str,
    platform:   &str,
    context:    Option<&str>,
    model:      &Model,
    model_auto: bool,
    model_reason: Option<&str>,
) -> Result<AiAnalysis, String> {
    let context_block = context
        .map(|c| format!("\n\n**Extra context:**\n```json\n{}\n```", c))
        .unwrap_or_default();

    let depth_instruction = match model {
        Model::Haiku  => "Be concise. One-sentence answers where possible.",
        Model::Sonnet => "Be thorough but focused. 2–4 sentences per field.",
        Model::Opus   => "Be comprehensive. Trace the full call path, explain all contributing factors, and provide production-ready fix code.",
    };

    let prompt = format!(
        r#"You are a senior software engineer specializing in debugging production errors.

Analyze this error and respond with a JSON object (no markdown fences, raw JSON only).

**Error title:** {title}
**Platform:** {platform}
**Stack trace:**
```
{stacktrace}
```{context_block}

{depth_instruction}

Respond with exactly this JSON structure:
{{
  "root_cause": "one-sentence root cause",
  "explanation": "explanation of what went wrong and why",
  "fix_suggestion": "concise actionable fix instructions",
  "code_example": "code snippet showing the fix, or null",
  "severity": "critical | high | medium | low",
  "prevention": "tip to prevent this class of error in future, or null"
}}

Be specific to the actual stack trace. Do not hallucinate file names or functions not present in the trace."#
    );

    let body = json!({
        "model":      model.api_id(),
        "max_tokens": model.max_tokens(),
        "messages":   [{ "role": "user", "content": prompt }]
    });

    let resp = client
        .post(ANTHROPIC_API)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Anthropic API {status}: {text}"));
    }

    let resp_json: Value = resp.json().await.map_err(|e| format!("JSON parse: {e}"))?;

    let text = resp_json["content"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|c| c["text"].as_str())
        .ok_or("empty response from Claude")?;

    let clean = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: Value = serde_json::from_str(clean)
        .map_err(|e| format!("Claude returned invalid JSON: {e}\nRaw: {clean}"))?;

    Ok(AiAnalysis {
        root_cause:     parsed["root_cause"].as_str().unwrap_or("Unknown").to_string(),
        explanation:    parsed["explanation"].as_str().unwrap_or("").to_string(),
        fix_suggestion: parsed["fix_suggestion"].as_str().unwrap_or("").to_string(),
        code_example:   parsed["code_example"].as_str().map(String::from),
        severity:       parsed["severity"].as_str().unwrap_or("medium").to_string(),
        prevention:     parsed["prevention"].as_str().map(String::from),
        model:          model.display_name().to_string(),
        model_auto,
        model_reason:   model_reason.map(String::from),
    })
}
