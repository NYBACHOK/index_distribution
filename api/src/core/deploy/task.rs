use std::time::Duration;

use anyhow::anyhow;
use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{
    accessors::cache::{KEY_PREFIX, deployed_node_key},
    core::types::RedeployTask,
    routes::node::Node,
    state::AppState,
};

const SLEEP_TIME_FOR_NEW_NODE: Duration = Duration::from_secs(60 * 5);

pub fn start_redeploy_task(
    state: AppState,
    mut receiver: tokio::sync::mpsc::Receiver<RedeployTask>,
) {
    let _ = tokio::spawn(async move {
        while let Some(task) = receiver.recv().await {
            let _ = tokio::spawn({
                let state = state.clone();
                let pool = state.pool.clone();

                async move {
                    let bundle_id = task.bundle_id;

                    if let Err(e) = handle_task(state, task).await {
                        tracing::error!("failed redeploy for {bundle_id}. Error: {e}");
                        let _ = set_undeployed(&pool, bundle_id).await
                            .inspect_err(|e | tracing::error!(error = %e, "failed to update status of deployment which failed to to deploy"));
                    }
                }
            });
        }
    });
}

async fn set_undeployed(pool: &sqlx::PgPool, bundle_id: Uuid) -> Result<(), anyhow::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query("update bundles set is_deployed = true where id = $1")
        .bind(bundle_id)
        .execute(&mut *transaction)
        .await?;

    Ok(())
}

async fn handle_task(
    state: AppState,
    RedeployTask { bundle_id }: RedeployTask,
) -> Result<(), anyhow::Error> {
    tracing::info!(bundle_id = %bundle_id, "Handling deployment task for bundle");

    loop {
        let mut connection = state.cache.get_multiplexed_async_connection().await?;

        let mut available_node = Option::None;

        for key in connection.keys(format!("{KEY_PREFIX}:*")).await? {
            let node: Node = serde_json::from_str(
                &connection
                    .get::<String>(key)
                    .await?
                    .ok_or(anyhow!("node with specified id"))?,
            )
            .map_err(|_| anyhow!("corrupted data"))?;

            if !connection.exists(deployed_node_key(node.id)).await? {
                let _ = available_node.insert(node);
            }
        }

        let available_node = match available_node {
            Some(n) => n,
            None => {
                tokio::time::sleep(SLEEP_TIME_FOR_NEW_NODE).await;
                continue;
            }
        };

        tracing::info!(bundle_id = %bundle_id, available_node_id = %available_node.id, "Found node for bundle");

        super::send_bundle_url(&state, bundle_id, available_node.clone()).await?;

        tracing::info!(bundle_id = %bundle_id, available_node_id = %available_node.id, "Send request to bundle");
    }
}
