use axum::extract::State;
use uuid::Uuid;

use crate::{
    accessors::cache::{CacheAccessor, FindBy},
    core::types::RedeployTask,
    errors::RouteError,
    routes::node::NodeManager,
    state::AppState,
    utils::json_extractor::Json,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DisconnectNode {
    pub id: Uuid,
}

#[utoipa::path(
    put,
    path = "/node/disconnect",
    request_body = crate::routes::node::DisconnectNode,
    responses(
        (status = 200, description = "Disconnected"),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn disconnect(
    _manager: NodeManager,
    State(state): State<AppState>,
    Json(DisconnectNode { id: node_id }): Json<DisconnectNode>,
) -> Result<(), RouteError> {
    state.cache.node_del(node_id).await?;

    let bundle_id = state
        .cache
        .deployed_bundle_del_by(FindBy::Node(node_id))
        .await?;

    let _ = state.redeploy_chanel.send(RedeployTask { bundle_id }).await;

    Ok(())
}
