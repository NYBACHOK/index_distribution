mod task;

use std::time::Duration;

pub use task::*;
use uuid::Uuid;

use crate::{
    accessors::cache::CacheAccessor, errors::RouteError, routes::node::Node, state::AppState,
};

const FILES_AVAILABLE: Duration = Duration::from_hours(1);

#[derive(Debug, serde::Deserialize)]
struct NodeResponse {
    status: String,
    #[allow(dead_code)]
    message: String,
}

pub async fn send_bundle_url(
    state: &AppState,
    bundle_id: Uuid,
    Node { url, kind, id, .. }: Node,
) -> Result<(), RouteError> {
    const MAX_RETRY_COUNT: u8 = 5;
    let mut retry_counts = 0;

    let archive = state
        .bucket
        .presign_get(
            format!("{}.zip", bundle_id),
            FILES_AVAILABLE.as_secs() as u32,
            None,
        )
        .await?;

    loop {
        retry_counts += 1;

        let response = state
            .http_client
            .put(url.join("/bundle").expect("valid url"))
            .body(serde_json::json!({ "bundle_link" : archive, "kind" : kind }).to_string())
            .send()
            .await?
            .error_for_status()?
            .json::<NodeResponse>()
            .await?;

        if response.status != "success" {
            if retry_counts > MAX_RETRY_COUNT {
                Err(anyhow::anyhow!(
                    "exceeded max retries for checking deployment status"
                ))?;
            }

            continue;
        } else {
            break;
        }
    }

    state.cache.deployed_bundle_set(bundle_id, id).await?;

    Ok(())
}
