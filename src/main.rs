mod config;
mod error;
mod middleware;
mod routes;

use crate::config::AppConfig;
use crate::routes::router;
use axum::Router;
use std::net::SocketAddr;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize logging/tracing first
    init_tracing();

    // Load configuration from environment
    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(error = %e, "failed to load configuration");
            std::process::exit(1);
        }
    };

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("invalid HOST/PORT combination");

    let app: Router = router(config.clone());

    info!(%addr, "starting server");
    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!(error = %e, "server error");
    }
}

fn init_tracing() {
    // Load .env if present for local development convenience
    let _ = dotenvy::dotenv();
    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,axum=info,tower_http=info".into());
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    use tokio::signal;

    #[cfg(unix)]
    {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    }
}
