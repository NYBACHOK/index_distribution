use std::sync::Arc;

use crate::StartError;

#[derive(Clone)]
pub struct AppState {
    pub bucket: Arc<Box<s3::Bucket>>,
}

impl AppState {
    pub async fn try_new(bucket: Box<s3::Bucket>) -> Result<Self, StartError> {
        Ok(Self {
            bucket: Arc::new(bucket),
        })
    }
}
