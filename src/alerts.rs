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
        SELECT id, channel, endpoint, cooldown_minutes, last_alerted_at
        FROM alerts
        WHERE project_id = $1 AND enabled = true
        "#,
        ctx.project_id
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    if alerts.is_empty() {
        return;
    }

    let event_type = if ctx.is_regression { "🔄 Regression" } else { "🚨 New Issue" };
    let message = format!(
        "[DevPulse] {} in {}\n{}: {}",
        event_type, ctx.project_name, ctx.level.to_uppercase(), ctx.title
    );

    let http = Client::new();

    for alert in alerts {
        let channel  = alert.channel.as_deref().unwrap_or("");
        let endpoint = alert.endpoint.clone().unwrap_or_default();

        // ── Cooldown check ────────────────────────────────────────────────────
        let cooldown_minutes = alert.cooldown_minutes.unwrap_or(60) as i64;
        if let Some(last) = alert.last_alerted_at {
            let elapsed = (chrono::Utc::now() - last).num_minutes();
            if elapsed < cooldown_minutes {
                tracing::info!(
                    "Alert '{}' on cooldown ({}/{} min elapsed), skipping",
                    channel, elapsed, cooldown_minutes
                );
                continue;
            }
        }

        // ── Fire ──────────────────────────────────────────────────────────────
        let fired = match channel {
            "webhook"  => { fire_webhook(&http, &endpoint, &ctx, &message).await; true }
            "telegram" => { fire_telegram(&http, &endpoint, &message).await; true }
            "email"    => { fire_email(&endpoint, &message, &ctx.title).await; true }
            other      => {
                tracing::warn!("Unknown alert channel: {}", other);
                false
            }
        };

        // ── Stamp last_alerted_at so the cooldown window starts now ───────────
        if fired {
            if let Err(e) = sqlx::query!(
                "UPDATE alerts SET last_alerted_at = NOW() WHERE id = $1",
                alert.id
            )
            .execute(pool)
            .await
            {
                tracing::error!("Failed to stamp last_alerted_at: {}", e);
            }
        }
    }
}

// ── Webhook ───────────────────────────────────────────────────────────────────
async fn fire_webhook(http: &Client, url: &str, ctx: &AlertContext, message: &str) {
    let body = serde_json::json!({
        "event":        if ctx.is_regression { "regression" } else { "new_issue" },
        "issue_id":     ctx.issue_id,
        "project_id":   ctx.project_id,
        "project_name": ctx.project_name,
        "title":        ctx.title,
        "level":        ctx.level,
        "message":      message,
    });

    match http.post(url).json(&body).send().await {
        Ok(r)  => tracing::info!("Webhook fired → {} ({})", url, r.status()),
        Err(e) => tracing::error!("Webhook failed → {}: {}", url, e),
    }
}

// ── Telegram ──────────────────────────────────────────────────────────────────
// endpoint format: "BOT_TOKEN:CHAT_ID"
async fn fire_telegram(http: &Client, endpoint: &str, message: &str) {
    let parts: Vec<&str> = endpoint.splitn(2, ':').collect();
    if parts.len() != 2 {
        tracing::error!("Invalid Telegram endpoint format. Use BOT_TOKEN:CHAT_ID");
        return;
    }
    let (bot_token, chat_id) = (parts[0], parts[1]);
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let body = serde_json::json!({
        "chat_id":    chat_id,
        "text":       message,
        "parse_mode": "HTML"
    });

    match http.post(&url).json(&body).send().await {
        Ok(r)  => tracing::info!("Telegram alert sent ({})", r.status()),
        Err(e) => tracing::error!("Telegram alert failed: {}", e),
    }
}

// ── Email ─────────────────────────────────────────────────────────────────────
// endpoint = recipient email; SMTP config from env vars
async fn fire_email(to: &str, body: &str, subject: &str) {
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_default();
    if smtp_host.is_empty() {
        tracing::debug!("SMTP_HOST not configured — skipping email alert");
        return;
    }

    let smtp_port = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".into())
        .parse::<u16>()
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER").unwrap_or_default();
    let smtp_pass = std::env::var("SMTP_PASS").unwrap_or_default();
    let from_addr = std::env::var("SMTP_FROM")
        .unwrap_or_else(|_| "devpulse@localhost".into());

    let from_mailbox = match from_addr.parse() {
        Ok(m)  => m,
        Err(e) => { tracing::error!("Invalid SMTP_FROM '{}': {}", from_addr, e); return; }
    };
    let to_mailbox = match to.parse() {
        Ok(m)  => m,
        Err(e) => { tracing::error!("Invalid recipient '{}': {}", to, e); return; }
    };

    let email = match Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(format!("[DevPulse] {}", subject))
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
    {
        Ok(e)  => e,
        Err(e) => { tracing::error!("Failed to build email: {}", e); return; }
    };

    let transport = if smtp_user.is_empty() {
        // Dev mode — no auth (Mailpit/Mailhog)
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host)
            .port(smtp_port)
            .build()
    } else {
        let creds = Credentials::new(smtp_user, smtp_pass);
        match AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host) {
            Ok(b)  => b.credentials(creds).build(),
            Err(e) => { tracing::error!("Failed to build SMTP transport: {}", e); return; }
        }
    };

    match transport.send(email).await {
        Ok(_)  => tracing::info!("Email alert sent to {}", to),
        Err(e) => tracing::error!("Email alert failed: {}", e),
    }
}
