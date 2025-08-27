use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
pub use sqlx::PgPool;
use tracing::info;

pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    info!("running migrations");
    sqlx::migrate!("../../migrations").run(pool).await?;
    Ok(())
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct Exchange {
    pub id: i32,
    pub name: String,
}

pub async fn insert_exchange(pool: &PgPool, name: &str) -> Result<Exchange> {
    let rec = sqlx::query_as::<_, Exchange>(
        "INSERT INTO exchanges (name) VALUES ($1) RETURNING id, name",
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn list_markets(pool: &PgPool) -> Result<Vec<String>> {
    let rows = sqlx::query!("SELECT symbol FROM markets")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| r.symbol).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_insert_exchange(pool: PgPool) -> Result<()> {
        let ex = insert_exchange(&pool, "hyperliquid").await?;
        assert_eq!(ex.name, "hyperliquid");
        Ok(())
    }
}
