use axum::extract::State;

use crate::{
    accessors::cache::CacheAccessor,
    errors::RouteError,
    routes::node::{Node, NodeManager},
    state::AppState,
    utils::json_extractor::Json,
};

pub async fn connect(
    _manager: NodeManager,
    State(state): State<AppState>,
    Json(node): Json<Node>,
) -> Result<(), RouteError> {
    state.cache.node_set(&node).await?;

    Ok(())
}
