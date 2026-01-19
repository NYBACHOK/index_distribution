use std::str::FromStr;

use axum::extract::{Query, State};
use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{
    KEY_HEADER_NAME, KEY_TO_MOBILE_APP,
    errors::RouteError,
    routes::{UuidQuery, node::Node},
    state::AppState,
    utils::jwt_auth::UserCredentials,
};

pub async fn status(
    _user: UserCredentials,
    State(state): State<AppState>,
    Query(UuidQuery { id: bundle_id }): Query<UuidQuery>,
) -> Result<String, RouteError> {
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

    let response = state
        .http_client
        .get(url.join("/bundle").unwrap())
        .header(KEY_HEADER_NAME, KEY_TO_MOBILE_APP)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
