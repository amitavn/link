use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json,
    Router
};
use serde::Deserialize;
use tower_service::Service;
use worker::*;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

fn router(kv: KvStore) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/shorten", post(shorten))
        .layer(Extension(kv))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let kv = env.kv("KV")?;
    Ok(router(kv).call(req).await?)
}

pub async fn root() -> &'static str {
    "hello, world!"
}

pub async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

#[worker::send]
pub async fn shorten(Extension(kv): Extension<KvStore>, Json(req): Json<ShortenRequest>) -> axum::Json<serde_json::Value> {
    // TODO
    axum::Json(serde_json::json!({ "url": req.url }))
}
