use axum::extract::State;

use crate::{
    accessors::cache::CacheAccessor,
    errors::RouteError,
    routes::node::{Node, NodeManager},
    state::AppState,
    utils::json_extractor::Json,
};

#[utoipa::path(
    put,
    path = "/node/connect",
    request_body = crate::routes::node::Node,
    responses(
        (status = 200, description = "Connected"),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn connect(
    _manager: NodeManager,
    State(state): State<AppState>,
    Json(node): Json<Node>,
) -> Result<(), RouteError> {
    state.cache.node_set(&node).await?;

    Ok(())
}
