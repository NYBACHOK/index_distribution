use std::time::Duration;

use axum::extract::State;

use crate::{
    accessors::cache::CacheAccessor,
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
    let Node { url, .. } = state.cache.node(node_id).await?;

    let mut transaction = state.pool.begin().await?;

    sqlx::query("update bundles set is_deployed = true where id == $1 and owner == $2")
        .bind(bundle_id)
        .bind(user.user_id)
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
        .put(url.join("/bundle").unwrap())
        .body(serde_json::json!({ "bundle_link" : archive}).to_string())
        .send()
        .await?;

    state.cache.deployed_bundle_set(bundle_id, node_id).await?;

    transaction.commit().await?;

    Ok(())
}
