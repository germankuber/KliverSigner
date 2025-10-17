use axum::{middleware, Router};

use crate::{config::AppConfig, middleware::require_api_key::require_api_key};
use crate::routes::sign::signers_routes;

mod health;
mod sign;
pub use health::health_routes;
pub use sign::sign_routes;

pub fn router(config: AppConfig) -> Router {
    // Public endpoints
    let public = health_routes();

    // Protected endpoints (require Authorization: ApiKey <key>)
    let protected = Router::new()
        .merge(sign_routes())
        .merge(signers_routes())
        .layer(middleware::from_fn_with_state(config.clone(), require_api_key))
        .with_state(config.clone());

    Router::new()
        .merge(public)
        .merge(protected)
}
