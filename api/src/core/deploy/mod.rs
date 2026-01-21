mod task;

use std::time::Duration;

pub use task::*;
use uuid::Uuid;

use crate::{
    accessors::cache::CacheAccessor, errors::RouteError, routes::node::Node, state::AppState,
};

const FILES_AVAILABLE: Duration = Duration::from_hours(1);

pub async fn send_bundle_url(
    state: &AppState,
    bundle_id: Uuid,
    node_id: Uuid,
) -> Result<(), RouteError> {
    let archive = state
        .bucket
        .presign_get(
            format!("{}.zip", bundle_id),
            FILES_AVAILABLE.as_secs() as u32,
            None,
        )
        .await?;

    let Node { url, .. } = state.cache.node(node_id).await?;

    state
        .http_client
        .put(url.join("/bundle").expect("valid url"))
        .body(serde_json::json!({ "bundle_link" : archive }).to_string())
        .send()
        .await?;

    state.cache.deployed_bundle_set(bundle_id, node_id).await?;

    Ok(())
}
