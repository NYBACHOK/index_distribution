use axum::extract::{Query, State};

use crate::{
    accessors::cache::{CacheAccessor, FindBy},
    errors::RouteError,
    routes::{UuidQuery, node::Node},
    state::AppState,
    utils::jwt_auth::UserCredentials,
};

#[utoipa::path(
    delete,
    path = "/deploy/delete",
    params(("id" = uuid::Uuid, Query, description = "Bundle id")),
    responses(
        (status = 200, description = "Deleted"),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
        (status = 404, description = "Not found", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn delete(
    user: UserCredentials,
    State(state): State<AppState>,
    Query(UuidQuery { id: bundle_id }): Query<UuidQuery>,
) -> Result<(), RouteError> {
    let node_id = state
        .cache
        .deployed_bundle(FindBy::Bundle(bundle_id))
        .await?;

    let Node { url, .. } = state.cache.node(node_id).await?;

    let mut transaction = state.pool.begin().await?;

    sqlx::query("update bundles set is_deployed = false where id = $1 and owner = $2")
        .bind(bundle_id)
        .bind(user.user_id)
        .execute(&mut *transaction)
        .await?;

    state
        .http_client
        .delete(url.join("/bundle").expect("valid url"))
        .send()
        .await?
        .error_for_status()?;

    state
        .cache
        .deployed_bundle_del_by(FindBy::Bundle(bundle_id))
        .await?;

    transaction.commit().await?;

    Ok(())
}
