use axum::extract::State;
use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{
    errors::RouteError,
    routes::node::{Node, NodeManager},
    state::AppState,
    utils::json_extractor::Json,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisconnectNode {
    pub id: Uuid,
}

pub async fn disconnect(
    _manager: NodeManager,
    State(state): State<AppState>,
    Json(DisconnectNode { id }): Json<DisconnectNode>,
) -> Result<(), RouteError> {
    let mut connection = state.cache.get_multiplexed_async_connection().await?;

    connection
        .del(format!("{}:{}", Node::KEY_PREFIX, id))
        .await?;

    // TODO: trigger redeploy

    Ok(())
}
