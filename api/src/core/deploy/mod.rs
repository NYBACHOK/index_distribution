mod task;

use std::time::Duration;

use mime::APPLICATION_JSON;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
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

    // Expose S3 to mobile
    // let archive = archive.replace(
    //     "http://localhost:9000",
    //     "NGROK OR ANY OTHER VARIANT",
    // );

    let body = serde_json::json!({ "bundle_link" : archive, "kind" : kind }).to_string();

    loop {
        retry_counts += 1;

        let response = state
            .http_client
            .put(url.join("/bundle").expect("valid url"))
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .header(CONTENT_LENGTH, body.len())
            .body(body.clone())
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
