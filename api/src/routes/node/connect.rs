use axum::extract::State;
use redis::AsyncTypedCommands;

use crate::{
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
    let mut connection = state.cache.get_multiplexed_async_connection().await?;

    connection
        .set(
            format!("{}:{}", Node::KEY_PREFIX, node.id),
            serde_json::to_string(&node).expect("serde can't fail"),
        )
        .await?;

    Ok(())
}
