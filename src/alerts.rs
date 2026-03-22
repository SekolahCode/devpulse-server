use reqwest::Client;
use sqlx::PgPool;
use uuid::Uuid;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Debug)]
pub struct AlertContext {
    pub issue_id:      Uuid,
    pub project_id:    Uuid,
    pub project_name:  String,
    pub title:         String,
    pub level:         String,
    pub is_regression: bool,
}

pub async fn fire_alerts(pool: &PgPool, ctx: AlertContext) {
    let alerts = sqlx::query!(
        r#"
        SELECT id, channel, endpoint, cooldown_minutes, last_alerted_at, retry_count
        FROM alerts
        WHERE project_id = $1
          AND enabled = true
          AND (retry_after IS NULL OR retry_after < NOW())
        "#,
        ctx.project_id
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    if alerts.is_empty() { return; }

    let event_type = if ctx.is_regression { "🔄 Regression" } else { "🚨 New Issue" };
    let message = format!(
        "[DevPulse] {} in {}\n{}: {}",
        event_type, ctx.project_name, ctx.level.to_uppercase(), ctx.title
    );

    let http = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    for alert in alerts {
        let channel  = alert.channel.as_deref().unwrap_or("");
        let endpoint = alert.endpoint.clone().unwrap_or_default();

        // Cooldown check
        let cooldown_minutes = alert.cooldown_minutes.unwrap_or(60) as i64;
        if let Some(last) = alert.last_alerted_at {
            let elapsed = (chrono::Utc::now() - last).num_minutes();
            if elapsed < cooldown_minutes {
                tracing::info!(
                    "Alert '{}' on cooldown ({}/{} min), skipping",
                    channel, elapsed, cooldown_minutes
                );
                continue;
            }
        }

        let result = match channel {
            "webhook"  => fire_webhook(&http, &endpoint, &ctx, &message).await,
            "telegram" => fire_telegram(&http, &endpoint, &message).await,
            "email"    => fire_email(&endpoint, &message, &ctx.title).await,
            other      => {
                tracing::warn!("Unknown alert channel: {}", other);
                Ok(())
            }
        };

        match result {
            Ok(()) => {
                // Success — stamp cooldown, reset retry counter
                if let Err(e) = sqlx::query!(
                    "UPDATE alerts SET last_alerted_at = NOW(), retry_count = 0,
                                       retry_after = NULL, last_error = NULL
                     WHERE id = $1",
                    alert.id
                )
                .execute(pool)
                .await
                {
                    tracing::error!("Failed to stamp last_alerted_at: {}", e);
                }
            }
            Err(err) => {
                // Failed — schedule exponential-backoff retry (max 5 attempts)
                let new_count = alert.retry_count + 1;
                let backoff_mins: i64 = match new_count {
                    1 => 2,
                    2 => 5,
                    3 => 15,
                    4 => 60,
                    _ => {
                        tracing::error!("Alert {} failed {} times, giving up: {}", alert.id, new_count, err);
                        // Disable after 5 consecutive failures
                        let _ = sqlx::query!(
                            "UPDATE alerts SET enabled = false, last_error = $1 WHERE id = $2",
                            err, alert.id
                        )
                        .execute(pool)
                        .await;
                        continue;
                    }
                };

                let retry_after = chrono::Utc::now() + chrono::Duration::minutes(backoff_mins);
                tracing::warn!(
                    "Alert {} failed (attempt {}), retrying in {}m: {}",
                    alert.id, new_count, backoff_mins, err
                );
                let _ = sqlx::query!(
                    "UPDATE alerts SET retry_count = $1, retry_after = $2, last_error = $3 WHERE id = $4",
                    new_count,
                    retry_after,
                    err,
                    alert.id
                )
                .execute(pool)
                .await;
            }
        }
    }
}

// ── Webhook ───────────────────────────────────────────────────────────────────
async fn fire_webhook(http: &Client, url: &str, ctx: &AlertContext, message: &str) -> Result<(), String> {
    let body = serde_json::json!({
        "event":        if ctx.is_regression { "regression" } else { "new_issue" },
        "issue_id":     ctx.issue_id,
        "project_id":   ctx.project_id,
        "project_name": ctx.project_name,
        "title":        ctx.title,
        "level":        ctx.level,
        "message":      message,
    });

    let resp = http.post(url).json(&body).send().await
        .map_err(|e| format!("Webhook request failed: {e}"))?;

    if resp.status().is_success() {
        tracing::info!("Webhook fired → {} ({})", url, resp.status());
        Ok(())
    } else {
        Err(format!("Webhook returned {}", resp.status()))
    }
}

// ── Telegram ──────────────────────────────────────────────────────────────────
// endpoint format: "BOT_TOKEN:CHAT_ID"
async fn fire_telegram(http: &Client, endpoint: &str, message: &str) -> Result<(), String> {
    let (bot_token, chat_id) = endpoint.split_once(':')
        .ok_or("Invalid Telegram endpoint: expected BOT_TOKEN:CHAT_ID")?;

    // Validate chat_id contains only digits (and optional leading -)
    if !chat_id.trim_start_matches('-').chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("Invalid Telegram chat_id: '{}'", chat_id));
    }

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let body = serde_json::json!({
        "chat_id":    chat_id,
        "text":       message,
        "parse_mode": "HTML"
    });

    let resp = http.post(&url).json(&body).send().await
        .map_err(|e| format!("Telegram request failed: {e}"))?;

    if resp.status().is_success() {
        tracing::info!("Telegram alert sent ({})", resp.status());
        Ok(())
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(format!("Telegram API error: {body}"))
    }
}

// ── Email ─────────────────────────────────────────────────────────────────────
async fn fire_email(to: &str, body: &str, subject: &str) -> Result<(), String> {
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_default();
    if smtp_host.is_empty() {
        tracing::debug!("SMTP_HOST not configured — skipping email alert");
        return Ok(()); // not a failure — just not configured
    }

    let smtp_port = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".into())
        .parse::<u16>()
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = std::env::var("SMTP_PASS").unwrap_or_default();
    let from_addr = std::env::var("SMTP_FROM")
        .unwrap_or_else(|_| "devpulse@localhost".into());

    let from_mailbox = from_addr.parse()
        .map_err(|e| format!("Invalid SMTP_FROM '{}': {}", from_addr, e))?;
    let to_mailbox = to.parse()
        .map_err(|e| format!("Invalid recipient '{}': {}", to, e))?;

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(format!("[DevPulse] {}", subject))
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| format!("Failed to build email: {e}"))?;

    let transport = if smtp_user.is_empty() {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host)
            .port(smtp_port)
            .build()
    } else {
        let creds = Credentials::new(smtp_user, smtp_pass);
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(|e| format!("SMTP relay error: {e}"))?
            .credentials(creds)
            .build()
    };

    transport.send(email).await
        .map_err(|e| format!("Email send failed: {e}"))?;

    tracing::info!("Email alert sent to {}", to);
    Ok(())
}
