use dotenvy::dotenv;
use std::env;
use tokio::time::{interval, Duration};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = storage::init_pool(&db_url).await?;
    storage::migrate(&pool).await?;

    let mut spread_iv = interval(Duration::from_secs(5));
    let mut liq_iv = interval(Duration::from_secs(60));
    let mut alert_iv = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = spread_iv.tick() => {
                info!("spread snapshot tick");
            }
            _ = liq_iv.tick() => {
                info!("liq risk refresh");
            }
            _ = alert_iv.tick() => {
                let alerts = alert::evaluate_rules(&pool).await?;
                for a in alerts { let _ = alert::send_alert("", &a).await; }
            }
        }
    }
}
