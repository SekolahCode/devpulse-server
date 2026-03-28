/// Multi-provider AI client for issue analysis.
/// Supports Anthropic (Claude), OpenAI (GPT), and Google (Gemini).
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const ANTHROPIC_API: &str = "https://api.anthropic.com/v1/messages";
const OPENAI_API:    &str = "https://api.openai.com/v1/chat/completions";

// ── Provider ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    Anthropic,
    OpenAI,
    Google,
}

// ── Complexity tier ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Tier {
    Fast,     // cheap/quick — vitals, info, short stacks, high-frequency
    Balanced, // standard production errors
    Deep,     // chained exceptions, long stacks, critical errors
}

// ── Model registry ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    // Anthropic Claude
    ClaudeHaiku,
    ClaudeSonnet,
    ClaudeOpus,
    // OpenAI
    Gpt4oMini,
    Gpt4o,
    // Google Gemini
    GeminiFlash,
    GeminiPro,
}

impl Model {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            // Anthropic — short and full IDs
            "haiku"  | "claude-haiku"  | "claude-haiku-4-5"           => Some(Self::ClaudeHaiku),
            "sonnet" | "claude-sonnet" | "claude-sonnet-4-6"           => Some(Self::ClaudeSonnet),
            "opus"   | "claude-opus"   | "claude-opus-4-6"             => Some(Self::ClaudeOpus),
            // OpenAI
            "gpt-4o-mini" | "gpt4o-mini"                               => Some(Self::Gpt4oMini),
            "gpt-4o"      | "gpt4o"                                    => Some(Self::Gpt4o),
            // Google
            "gemini-flash" | "gemini-2.0-flash" | "gemini-2.0-flash-exp" => Some(Self::GeminiFlash),
            "gemini-pro"   | "gemini-1.5-pro"                          => Some(Self::GeminiPro),
            _ => None,
        }
    }

    pub fn provider(&self) -> Provider {
        match self {
            Self::ClaudeHaiku | Self::ClaudeSonnet | Self::ClaudeOpus => Provider::Anthropic,
            Self::Gpt4oMini   | Self::Gpt4o                          => Provider::OpenAI,
            Self::GeminiFlash | Self::GeminiPro                      => Provider::Google,
        }
    }

    pub fn tier(&self) -> Tier {
        match self {
            Self::ClaudeHaiku | Self::Gpt4oMini | Self::GeminiFlash => Tier::Fast,
            Self::ClaudeSonnet | Self::Gpt4o | Self::GeminiPro      => Tier::Balanced,
            Self::ClaudeOpus                                         => Tier::Deep,
        }
    }

    pub fn api_id(&self) -> &'static str {
        match self {
            Self::ClaudeHaiku  => "claude-haiku-4-5-20251001",
            Self::ClaudeSonnet => "claude-sonnet-4-6",
            Self::ClaudeOpus   => "claude-opus-4-6",
            Self::Gpt4oMini    => "gpt-4o-mini",
            Self::Gpt4o        => "gpt-4o",
            Self::GeminiFlash  => "gemini-2.0-flash",
            Self::GeminiPro    => "gemini-1.5-pro",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ClaudeHaiku  => "Claude Haiku 4.5",
            Self::ClaudeSonnet => "Claude Sonnet 4.6",
            Self::ClaudeOpus   => "Claude Opus 4.6",
            Self::Gpt4oMini    => "GPT-4o mini",
            Self::Gpt4o        => "GPT-4o",
            Self::GeminiFlash  => "Gemini 2.0 Flash",
            Self::GeminiPro    => "Gemini 1.5 Pro",
        }
    }

    pub fn max_tokens(&self) -> u32 {
        match self {
            Self::ClaudeHaiku | Self::Gpt4oMini | Self::GeminiFlash => 512,
            Self::ClaudeSonnet | Self::Gpt4o | Self::GeminiPro      => 1024,
            Self::ClaudeOpus                                         => 2048,
        }
    }
}

// ── Available API keys ────────────────────────────────────────────────────────

pub struct ApiKeys<'a> {
    pub anthropic: Option<&'a str>,
    pub openai:    Option<&'a str>,
    pub gemini:    Option<&'a str>,
}

impl ApiKeys<'_> {
    pub fn has_anthropic(&self) -> bool { self.anthropic.is_some() }
    pub fn has_openai(&self)    -> bool { self.openai.is_some() }
    pub fn has_gemini(&self)    -> bool { self.gemini.is_some() }
    pub fn any(&self)           -> bool { self.has_anthropic() || self.has_openai() || self.has_gemini() }
}

// ── Auto-selection logic ──────────────────────────────────────────────────────

pub struct AnalysisContext {
    pub stacktrace_frames:   usize,
    pub level:               String,
    pub event_count:         i64,
    pub has_exception_chain: bool,
    pub is_vitals:           bool,
}

/// Pick the best available model for the issue context.
/// Determines required tier from signals, then picks the best available
/// model in that tier (preference: Anthropic > OpenAI > Google).
pub fn select_model<'a>(ctx: &AnalysisContext, keys: &ApiKeys<'a>) -> (Model, &'static str) {
    let tier   = required_tier(ctx);
    let reason = tier_reason(ctx);

    let model = match tier {
        Tier::Fast => {
            if keys.has_anthropic() { Model::ClaudeHaiku }
            else if keys.has_openai() { Model::Gpt4oMini }
            else { Model::GeminiFlash }
        }
        Tier::Balanced => {
            if keys.has_anthropic() { Model::ClaudeSonnet }
            else if keys.has_openai() { Model::Gpt4o }
            else { Model::GeminiPro }
        }
        Tier::Deep => {
            if keys.has_anthropic() { Model::ClaudeOpus }
            else if keys.has_openai() { Model::Gpt4o }
            else { Model::GeminiPro }
        }
    };

    (model, reason)
}

fn required_tier(ctx: &AnalysisContext) -> Tier {
    if ctx.is_vitals                                                  { return Tier::Fast; }
    if ctx.has_exception_chain                                        { return Tier::Deep; }
    if ctx.stacktrace_frames > 30                                     { return Tier::Deep; }
    if ctx.level == "error" && ctx.stacktrace_frames > 20            { return Tier::Deep; }
    if ctx.level == "info"                                            { return Tier::Fast; }
    if ctx.stacktrace_frames < 5                                      { return Tier::Fast; }
    if ctx.event_count > 500                                          { return Tier::Fast; }
    Tier::Balanced
}

fn tier_reason(ctx: &AnalysisContext) -> &'static str {
    if ctx.is_vitals {
        "Performance vitals — metrics interpretation doesn't need deep reasoning"
    } else if ctx.has_exception_chain {
        "Chained exception detected — multi-layer cause analysis requires the most capable model"
    } else if ctx.stacktrace_frames > 30 {
        "Large stack trace — deep call-graph analysis benefits from extended reasoning"
    } else if ctx.level == "error" && ctx.stacktrace_frames > 20 {
        "Critical error with complex stack — thorough root-cause reasoning selected"
    } else if ctx.level == "info" {
        "Info-level event — lightweight analysis is sufficient"
    } else if ctx.stacktrace_frames < 5 {
        "Short stack trace — fast model is sufficient for simple errors"
    } else if ctx.event_count > 500 {
        "High-frequency issue — well-known error pattern, fast analysis appropriate"
    } else {
        "Standard production error — balanced model selected for accuracy and speed"
    }
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

// ── Prompt & response helpers ─────────────────────────────────────────────────

fn build_prompt(title: &str, stacktrace: &str, platform: &str, context: Option<&str>, model: &Model) -> String {
    let context_block = context
        .map(|c| format!("\n\n**Extra context:**\n```json\n{}\n```", c))
        .unwrap_or_default();

    let depth_instruction = match model.tier() {
        Tier::Fast     => "Be concise. One-sentence answers where possible.",
        Tier::Balanced => "Be thorough but focused. 2–4 sentences per field.",
        Tier::Deep     => "Be comprehensive. Trace the full call path, explain all contributing factors, and provide production-ready fix code.",
    };

    format!(
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
    )
}

fn parse_ai_response(text: &str, model: &Model, model_auto: bool, model_reason: Option<&str>) -> Result<AiAnalysis, String> {
    let clean = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: Value = serde_json::from_str(clean)
        .map_err(|e| format!("AI returned invalid JSON: {e}\nRaw: {clean}"))?;

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

// ── Provider-specific API calls ───────────────────────────────────────────────

async fn call_anthropic(client: &Client, api_key: &str, prompt: &str, model: &Model) -> Result<String, String> {
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
    resp_json["content"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|c| c["text"].as_str())
        .map(String::from)
        .ok_or_else(|| "Empty response from Anthropic".into())
}

async fn call_openai(client: &Client, api_key: &str, prompt: &str, model: &Model) -> Result<String, String> {
    let body = json!({
        "model":      model.api_id(),
        "max_tokens": model.max_tokens(),
        "messages":   [{ "role": "user", "content": prompt }]
    });

    let resp = client
        .post(OPENAI_API)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("OpenAI API {status}: {text}"));
    }

    let resp_json: Value = resp.json().await.map_err(|e| format!("JSON parse: {e}"))?;
    resp_json["choices"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|c| c["message"]["content"].as_str())
        .map(String::from)
        .ok_or_else(|| "Empty response from OpenAI".into())
}

async fn call_gemini(client: &Client, api_key: &str, prompt: &str, model: &Model) -> Result<String, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model.api_id(),
        api_key
    );

    let body = json!({
        "contents": [{ "parts": [{ "text": prompt }] }],
        "generationConfig": { "maxOutputTokens": model.max_tokens() }
    });

    let resp = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini API {status}: {text}"));
    }

    let resp_json: Value = resp.json().await.map_err(|e| format!("JSON parse: {e}"))?;
    resp_json["candidates"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|c| c["content"]["parts"].as_array())
        .and_then(|p| p.first())
        .and_then(|p| p["text"].as_str())
        .map(String::from)
        .ok_or_else(|| "Empty response from Gemini".into())
}

// ── Public entry point ────────────────────────────────────────────────────────

pub async fn analyse_issue(
    client:       &Client,
    keys:         &ApiKeys<'_>,
    title:        &str,
    stacktrace:   &str,
    platform:     &str,
    context:      Option<&str>,
    model:        &Model,
    model_auto:   bool,
    model_reason: Option<&str>,
) -> Result<AiAnalysis, String> {
    let prompt = build_prompt(title, stacktrace, platform, context, model);

    let text = match model.provider() {
        Provider::Anthropic => {
            let key = keys.anthropic.ok_or("ANTHROPIC_API_KEY not configured")?;
            call_anthropic(client, key, &prompt, model).await?
        }
        Provider::OpenAI => {
            let key = keys.openai.ok_or("OPENAI_API_KEY not configured")?;
            call_openai(client, key, &prompt, model).await?
        }
        Provider::Google => {
            let key = keys.gemini.ok_or("GEMINI_API_KEY not configured")?;
            call_gemini(client, key, &prompt, model).await?
        }
    };

    parse_ai_response(&text, model, model_auto, model_reason)
}
