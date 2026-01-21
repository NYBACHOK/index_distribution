use axum::extract::State;
use uuid::Uuid;

use crate::{
    accessors::cache::CacheAccessor, core::types::RedeployTask, errors::RouteError,
    routes::node::NodeManager, state::AppState, utils::json_extractor::Json,
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
    state.cache.node_del(id).await?;

    let _ = state.redeploy_chanel.send(RedeployTask {}).await;

    Ok(())
}
