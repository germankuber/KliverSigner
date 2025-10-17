use axum::{routing::get, Json, Router};
use serde::Serialize;
use derive_builder::Builder;

// Health endpoint: cheap liveness probe
// Returns 200 OK with static service metadata
pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_handler))
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(name = "_build"))]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

impl HealthResponseBuilder {
    fn build(self) -> Result<HealthResponse, &'static str> {
        self._build().map_err(|_| "invalid response")
    }
}

async fn health_handler() -> Json<HealthResponse> {
    let resp = HealthResponseBuilder::default()
        .status("ok")
        .service(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .build()
        .expect("health response must be valid");
    Json(resp)
}
