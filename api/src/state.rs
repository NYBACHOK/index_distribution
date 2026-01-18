use std::sync::Arc;

use crate::{StartError, utils::jwt_auth::JwtKeys};

#[derive(Clone)]
pub struct AppState {
    pub bucket: Arc<Box<s3::Bucket>>,
    pub keys: Arc<JwtKeys>,
}

impl AppState {
    pub async fn try_new(bucket: Box<s3::Bucket>, keys: JwtKeys) -> Result<Self, StartError> {
        Ok(Self {
            bucket: Arc::new(bucket),
            keys: Arc::new(keys),
        })
    }
}
