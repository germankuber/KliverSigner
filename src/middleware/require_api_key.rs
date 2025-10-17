use crate::{config::AppConfig, error::AppError};
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use tracing::debug;

// Middleware that checks Authorization: ApiKey <key> header equals configured API key.
pub async fn require_api_key(
    State(config): axum::extract::State<AppConfig>,
    req: Request<Body>,
    next: Next,
)-> Result<Response, AppError> {
    use axum::http::header::AUTHORIZATION;

    let provided = req.headers().get(AUTHORIZATION).and_then(|v| v.to_str().ok());
    if let Some(auth) = provided {
        // Expect format: "ApiKey <key>"
        let mut parts = auth.splitn(2, ' ');
        let scheme = parts.next().unwrap_or("");
        let token = parts.next().unwrap_or("");
        if scheme.eq_ignore_ascii_case("ApiKey") && token == config.api_key {
            debug!("api key authorized");
            return Ok(next.run(req).await);
        }
    }
    Err(AppError::Unauthorized)
}

use axum::extract::State;
