use crate::core::types::RedeployTask;

pub fn start_redeploy_task(mut receiver: tokio::sync::mpsc::Receiver<RedeployTask>) {
    let _ = tokio::spawn(async move {
        while let Some(task) = receiver.recv().await {
            let _ = tokio::spawn(async move {
                let _ = handle_task(task).await;
            });
        }
    });
}

async fn handle_task(task: RedeployTask) {
    todo!()
}
