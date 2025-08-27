use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub title: String,
    pub body: String,
}

pub async fn send_alert(webhook: &str, alert: &Alert) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::json!({
        "text": format!("{}\n{}", alert.title, alert.body)
    });
    client.post(webhook).json(&payload).send().await?;
    Ok(())
}

pub async fn evaluate_rules(pool: &storage::PgPool) -> Result<Vec<Alert>> {
    // TODO: 실제 규칙 평가 로직
    let rows = sqlx::query!("SELECT rule_type, params FROM alert_rules WHERE enabled = true")
        .fetch_all(pool)
        .await?;
    let alerts = rows
        .into_iter()
        .map(|r| Alert {
            title: r.rule_type,
            body: r.params.unwrap_or_default().to_string(),
        })
        .collect();
    Ok(alerts)
}
