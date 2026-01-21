use anyhow::anyhow;

use crate::{core::types::RedeployTask, state::AppState};

pub fn start_redeploy_task(
    state: AppState,
    mut receiver: tokio::sync::mpsc::Receiver<RedeployTask>,
) {
    let _ = tokio::spawn(async move {
        while let Some(task) = receiver.recv().await {
            let _ = tokio::spawn({
                let state = state.clone();

                async move {
                    let bundle_id = task.bundle_id;

                    let _ = handle_task(state, task).await.inspect_err(|e| {
                        tracing::error!("failed redeploy for {bundle_id}. Error: {e}")
                    });
                }
            });
        }
    });
}

async fn handle_task(
    state: AppState,
    RedeployTask {
        bundle_id,
        node_id: old_node_id,
    }: RedeployTask,
) -> Result<(), anyhow::Error> {
    const MAX_RETRY_COUNT: u8 = 5;
    let mut retry_counts = 0;

    loop {
        if retry_counts > MAX_RETRY_COUNT {
            return Err(anyhow!(
                "exceeded max retries for checking deployment status"
            ));
        }

        retry_counts += 1;
    }
}
