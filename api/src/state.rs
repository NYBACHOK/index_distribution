use std::sync::Arc;

use crate::{StartError, utils::jwt_auth::JwtKeys};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub bucket: Arc<Box<s3::Bucket>>,
    pub keys: Arc<JwtKeys>,
}

impl AppState {
    pub async fn try_new(
        bucket: Box<s3::Bucket>,
        rsa_pub_key_base64: String,
        connection_string: String,
    ) -> Result<Self, StartError> {
        let pool = sqlx::postgres::PgPool::connect(connection_string.as_ref()).await?;

        let jwt_keys =
            JwtKeys::try_from_pem(data_encoding::BASE64.decode(rsa_pub_key_base64.as_bytes())?)?;

        Ok(Self {
            bucket: Arc::new(bucket),
            keys: Arc::new(jwt_keys),
            pool,
        })
    }
}
