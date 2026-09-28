mod config;
mod routes;
mod state;

use crate::config::Config;
use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("gitgat_control_plane=info,tower_http=info")),
        )
        .init();

    let config = Config::from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState::new(pool);
    let app = routes::router(state).layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(config.bind).await?;

    info!(address = %config.bind, "GitGat Control Plane listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut stream = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        stream.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
