use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use storage::{self, PgPool};
use analytics;
use alert;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(health, markets, spread, heatmap, liq_risk, pnl_rank, stable_metrics, create_alert),
    components(schemas(SpreadQuery, SpreadResp, HeatmapQuery, LiqRiskQuery, NewAlert, Alert)))]
struct ApiDoc;

pub fn build_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/markets", get(markets))
        .route("/spread", get(spread))
        .route("/heatmap", get(heatmap))
        .route("/liq-risk", get(liq_risk))
        .route("/pnl/rank", get(pnl_rank))
        .route("/stable/metrics", get(stable_metrics))
        .route("/alerts", post(create_alert))
        .with_state(pool.clone())
        .merge(SwaggerUi::new("/docs").url("/api-doc.json", ApiDoc::openapi()))
}

async fn health() -> &'static str {
    "OK"
}

async fn markets(State(pool): State<PgPool>) -> Json<Vec<String>> {
    let markets = storage::list_markets(&pool).await.unwrap_or_default();
    Json(markets)
}

#[derive(Deserialize, ToSchema)]
struct HeatmapQuery {
    symbol: String,
    bins: Option<usize>,
}

#[derive(Serialize, ToSchema)]
struct HeatmapResp {
    buckets: Vec<(f64, f64, i64)>,
}

async fn heatmap(_state: State<PgPool>, _q: axum::extract::Query<HeatmapQuery>) -> Json<HeatmapResp> {
    Json(HeatmapResp { buckets: vec![] })
}

#[derive(Deserialize, ToSchema)]
struct LiqRiskQuery {
    symbol: String,
}

#[derive(Serialize, ToSchema)]
struct LiqRiskResp {
    index: f64,
}

async fn liq_risk(_state: State<PgPool>, _q: axum::extract::Query<LiqRiskQuery>) -> Json<LiqRiskResp> {
    let positions = vec![];
    let (_, idx) = analytics::calc_liq_risk(&positions, 0.0, 10);
    Json(LiqRiskResp { index: idx })
}

#[derive(Deserialize, ToSchema)]
struct SpreadQuery {
    base: f64,
    reference: f64,
}

#[derive(Serialize, ToSchema)]
struct SpreadResp {
    spread_bp: f64,
}

async fn spread(Query(q): axum::extract::Query<SpreadQuery>) -> Json<SpreadResp> {
    let bp = analytics::calc_spread(q.base, q.reference);
    Json(SpreadResp { spread_bp: bp })
}

async fn pnl_rank() -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

async fn stable_metrics() -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

#[derive(Deserialize, ToSchema)]
struct NewAlert {
    rule_type: String,
    params: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
struct Alert {
    id: i64,
    rule_type: String,
}

async fn create_alert(
    State(_pool): State<PgPool>,
    Json(new): Json<NewAlert>,
) -> Json<Alert> {
    // TODO: DB에 저장
    Json(Alert { id: 1, rule_type: new.rule_type })
}
