use gitgat_control_plane::{app, config::Config, state::AppState};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    let config = Config::from_env()?;
    let pool = sqlx::PgPool::connect(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    let app = app(AppState { pool });

    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    info!(bind = %config.bind, "GitGat Control Plane listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

async fn shutdown() {
    let _ = tokio::signal::ctrl_c().await;
}
