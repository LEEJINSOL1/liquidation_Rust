use api::build_router;
use axum::{body::Body, http::{Request, StatusCode}};
use storage;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn health_ok() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/liquidscope")
        .expect("lazy pool");
    let app = build_router(pool);
    let resp = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn spread_endpoint() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/liquidscope")
        .expect("lazy pool");
    let app = build_router(pool);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/spread?base=100&reference=90")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
