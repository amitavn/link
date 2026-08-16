use axum::{routing::get, routing::post, Json, Router};
use serde::Deserialize;
use tower_service::Service;
use worker::*;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/shorten", post(shorten))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "hello, world!"
}

pub async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

pub async fn shorten(Json(req): Json<ShortenRequest>) -> axum::Json<serde_json::Value> {
    // TODO
    axum::Json(serde_json::json!({ "url": req.url }))
}
