use std::net::SocketAddr;

use axum::routing::get;
use tower_http::cors::CorsLayer;

use crate::state::AppState;

mod errors;
mod openapi;
mod state;

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("Failed to start server. Reason: {0}")]
    Io(#[from] std::io::Error),
}

async fn health() -> String {
    "Healthy".to_owned()
}

pub async fn start_api(host: SocketAddr) -> Result<(), StartError> {
    let state = AppState::try_new().await?;

    let routes = axum::Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    let app = axum::Router::new()
        .route("/health", get(health))
        .merge(routes)
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url(
            "/api-docs/openapi.json",
            <openapi::ApiDoc as utoipa::OpenApi>::openapi(),
        ))
        .layer(CorsLayer::very_permissive());

    let tcp_listener = tokio::net::TcpListener::bind(host).await?;

    tracing::info!("Listen at: {}", host);

    axum::serve(tcp_listener, app).await?;

    Ok(())
}
