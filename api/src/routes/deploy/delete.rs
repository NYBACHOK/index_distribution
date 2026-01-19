use std::str::FromStr;

use axum::extract::{Query, State};
use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{
    errors::RouteError,
    routes::{UuidQuery, node::Node},
    state::AppState,
    utils::jwt_auth::UserCredentials,
};

pub async fn delete(
    user: UserCredentials,
    State(state): State<AppState>,
    Query(UuidQuery { id: bundle_id }): Query<UuidQuery>,
) -> Result<(), RouteError> {
    let mut connection = state.cache.get_multiplexed_async_connection().await?;

    let node_id = connection
        .get(format!(
            "{}:{}",
            Node::DEPLOYED_BUNDLE_CACHE_PREFIX,
            bundle_id
        ))
        .await?
        .map(|this| Uuid::from_str(&this).ok())
        .flatten()
        .ok_or(RouteError::NotFound("node with this bundle"))?;

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

    state
        .http_client
        .delete(url.join("/bundle").unwrap())
        .send()
        .await?;

    connection
        .del(format!(
            "{}:{}",
            Node::DEPLOYED_BUNDLE_CACHE_PREFIX,
            bundle_id
        ))
        .await?;

    transaction.commit().await?;

    Ok(())
}
