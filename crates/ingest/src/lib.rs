use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeEvent {
    pub symbol: String,
    pub price: f64,
    pub qty: f64,
    pub side: String,
}

#[async_trait]
pub trait MarketFeed {
    async fn stream_trades(&self, symbol: &str, tx: mpsc::Sender<TradeEvent>) -> Result<()>;
    async fn latest_price(&self, symbol: &str) -> Result<f64>;
}

pub struct HyperliquidFeed {
    pub ws_url: String,
}

impl HyperliquidFeed {
    pub fn new(ws_url: String) -> Self {
        Self { ws_url }
    }
}

#[async_trait]
impl MarketFeed for HyperliquidFeed {
    async fn stream_trades(&self, symbol: &str, _tx: mpsc::Sender<TradeEvent>) -> Result<()> {
        info!(?symbol, "stream_trades Hyperliquid stub");
        Ok(()) // TODO: 실제 WS 연결
    }
    async fn latest_price(&self, _symbol: &str) -> Result<f64> {
        Ok(0.0) // TODO: REST 호출
    }
}

pub struct BinanceFeed {
    pub ws_url: String,
}

impl BinanceFeed {
    pub fn new(ws_url: String) -> Self {
        Self { ws_url }
    }
}

#[async_trait]
impl MarketFeed for BinanceFeed {
    async fn stream_trades(&self, symbol: &str, _tx: mpsc::Sender<TradeEvent>) -> Result<()> {
        info!(?symbol, "stream_trades Binance stub");
        Ok(())
    }
    async fn latest_price(&self, _symbol: &str) -> Result<f64> {
        Ok(0.0)
    }
}

pub struct BybitFeed {
    pub ws_url: String,
}

impl BybitFeed {
    pub fn new(ws_url: String) -> Self {
        Self { ws_url }
    }
}

#[async_trait]
impl MarketFeed for BybitFeed {
    async fn stream_trades(&self, symbol: &str, _tx: mpsc::Sender<TradeEvent>) -> Result<()> {
        info!(?symbol, "stream_trades Bybit stub");
        Ok(())
    }
    async fn latest_price(&self, _symbol: &str) -> Result<f64> {
        Ok(0.0)
    }
}

pub async fn consume_feed<F: MarketFeed + Send + Sync>(
    feed: F,
    symbol: &str,
    pool: storage::PgPool,
) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<TradeEvent>(100);
    let sym = symbol.to_string();
    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let _ = storage::insert_exchange(&pool, &evt.symbol).await; // placeholder
            info!(?evt, "consumed trade event");
        }
    });
    feed.stream_trades(&sym, tx).await
}
