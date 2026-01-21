use axum::extract::{Query, State};

use crate::{
    accessors::cache::CacheAccessor,
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
    let node_id = state.cache.deployed_bundle_node_id(bundle_id).await?;

    let Node { url, .. } = state.cache.node(node_id).await?;

    let response = state
        .http_client
        .get(url.join("/bundle").expect("valid url"))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
