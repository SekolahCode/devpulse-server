/// Claude API client for AI-powered issue analysis.
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const ANTHROPIC_API: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-6";

#[derive(Debug, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub root_cause:     String,
    pub explanation:    String,
    pub fix_suggestion: String,
    pub code_example:   Option<String>,
    pub severity:       String,
    pub prevention:     Option<String>,
    pub model:          String,
}

/// Analyse an error issue using Claude.
/// `title`      — issue title / exception type
/// `stacktrace` — formatted stack trace string
/// `platform`   — e.g. "php", "javascript", "rust"
/// `context`    — optional extra context JSON
pub async fn analyse_issue(
    client: &Client,
    api_key: &str,
    title: &str,
    stacktrace: &str,
    platform: &str,
    context: Option<&str>,
) -> Result<AiAnalysis, String> {
    let context_block = context
        .map(|c| format!("\n\n**Extra context:**\n```json\n{}\n```", c))
        .unwrap_or_default();

    let prompt = format!(
        r#"You are a senior software engineer specializing in debugging production errors.

Analyze this error and respond with a JSON object (no markdown fences, raw JSON only).

**Error title:** {title}
**Platform:** {platform}
**Stack trace:**
```
{stacktrace}
```{context_block}

Respond with exactly this JSON structure:
{{
  "root_cause": "one-sentence root cause",
  "explanation": "2-4 sentences explaining what went wrong and why",
  "fix_suggestion": "concise actionable fix instructions",
  "code_example": "optional code snippet showing the fix, or null",
  "severity": "critical | high | medium | low",
  "prevention": "optional tip to prevent this class of error in future, or null"
}}

Be specific to the actual stack trace. Do not hallucinate file names or functions not present in the trace."#
    );

    let body = json!({
        "model": MODEL,
        "max_tokens": 1024,
        "messages": [{ "role": "user", "content": prompt }]
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

    // Strip any accidental markdown code fences
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
        model:          MODEL.to_string(),
    })
}
