use std::time::Duration;

use axum::extract::State;
use redis::AsyncTypedCommands;

use crate::{
    errors::RouteError,
    routes::{deploy::DeployBundleModel, node::Node},
    state::AppState,
    utils::{json_extractor::Json, jwt_auth::UserCredentials},
};

const FILES_AVAILABLE: Duration = Duration::from_hours(1);

pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
    Json(DeployBundleModel { bundle_id, node_id }): Json<DeployBundleModel>,
) -> Result<(), RouteError> {
    let mut connection = state.cache.get_multiplexed_async_connection().await?;

    let Node { url, .. } = serde_json::from_str(
        &connection
            .get::<String>(format!("{}:{}", Node::KEY_PREFIX, node_id))
            .await?
            .ok_or(RouteError::NotFound("node with specified id"))?,
    )
    .map_err(|_| RouteError::Unexpected("corrupted data".to_owned()))?;

    let mut transaction = state.pool.begin().await?;

    sqlx::query("update bundles set is_deployed = true where id == $1 and owner == $2")
        .bind(bundle_id)
        .bind(user.user)
        .execute(&mut *transaction)
        .await?;

    let archive = state
        .bucket
        .presign_get(
            format!("{}.zip", bundle_id),
            FILES_AVAILABLE.as_secs() as u32,
            None,
        )
        .await?;

    state
        .http_client
        .put(url)
        .body(serde_json::json!({ "bundle_link" : archive}).to_string())
        .send()
        .await?;

    connection
        .set(
            format!("{}:{}", Node::DEPLOYED_BUNDLE_CACHE_PREFIX, node_id),
            bundle_id.to_string(),
        )
        .await?;

    transaction.commit().await?;

    Ok(())
}
