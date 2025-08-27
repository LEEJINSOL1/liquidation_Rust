use axum::Server;
use dotenvy::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = storage::init_pool(&db_url).await?;
    storage::migrate(&pool).await?;

    let app = api::build_router(pool);
    let addr = "0.0.0.0:3000".parse().unwrap();
    info!("listening", %addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
