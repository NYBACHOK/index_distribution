use axum::extract::State;

use crate::{
    core::deploy::send_bundle_url,
    errors::RouteError,
    routes::deploy::DeployBundleModel,
    state::AppState,
    utils::{json_extractor::Json, jwt_auth::UserCredentials},
};

pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
    Json(DeployBundleModel { bundle_id, node_id }): Json<DeployBundleModel>,
) -> Result<(), RouteError> {
    let mut transaction = state.pool.begin().await?;

    sqlx::query("update bundles set is_deployed = true where id == $1 and owner == $2")
        .bind(bundle_id)
        .bind(user.user_id)
        .execute(&mut *transaction)
        .await?;

    send_bundle_url(&state, bundle_id, node_id).await?;

    transaction.commit().await?;

    Ok(())
}
