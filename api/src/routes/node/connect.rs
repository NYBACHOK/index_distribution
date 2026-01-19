use axum::extract::State;
use redis::AsyncTypedCommands;
use url::Url;
use uuid::Uuid;

use crate::{
    errors::RouteError,
    routes::{
        NODE_PREFIX,
        node::{NodeKind, NodeManager},
    },
    state::AppState,
    utils::json_extractor::Json,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectNode {
    pub id: Uuid,
    /// Public url for already tunneled node
    pub url: Url,
    pub kind: NodeKind,
}

pub async fn connect(
    _manager: NodeManager,
    State(state): State<AppState>,
    Json(node): Json<ConnectNode>,
) -> Result<(), RouteError> {
    let mut connection = state.cache.get_multiplexed_async_connection().await?;

    connection
        .set(
            format!("{NODE_PREFIX}:{}", node.id),
            serde_json::to_string(&node).expect("serde can't fail"),
        )
        .await?;

    Ok(())
}
