use std::{ops::Deref, sync::Arc};

use reqwest::header;

use crate::{KEY_HEADER_NAME, StartError, core::types::RedeployTask, utils::jwt_auth::JwtKeys};

#[derive(Clone)]
pub struct AppState(Arc<InnerAppState>);

pub struct InnerAppState {
    pub pool: sqlx::PgPool,
    pub bucket: Box<s3::Bucket>,
    pub cache: redis::Client,
    pub keys: JwtKeys,
    pub http_client: reqwest::Client,
    pub redeploy_chanel: tokio::sync::mpsc::Sender<RedeployTask>,
    node_manager_password: &'static str,
}

impl InnerAppState {
    pub fn is_password_matches(&self, password: impl AsRef<str>) -> bool {
        self.node_manager_password == password.as_ref()
    }
}

impl AppState {
    pub async fn try_new(
        bucket: Box<s3::Bucket>,
        rsa_pub_key_base64: String,
        connection_string: String,
        redis_connection_string: String,
        node_manager_password: String,
        app_password: String,
        redeploy_chanel: tokio::sync::mpsc::Sender<RedeployTask>,
    ) -> Result<Self, StartError> {
        let pool = sqlx::postgres::PgPool::connect(connection_string.as_ref()).await?;

        let jwt_keys =
            JwtKeys::try_from_pem(data_encoding::BASE64.decode(rsa_pub_key_base64.as_bytes())?)?;

        let cache = redis::Client::open(redis_connection_string)?;

        let mut auth_header = header::HeaderValue::from_str(&app_password)
            .expect("invalid value to app password  header");
        auth_header.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(KEY_HEADER_NAME, auth_header);

        let http_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self(Arc::new(InnerAppState {
            pool,
            bucket,
            cache,
            keys: jwt_keys,
            http_client,
            redeploy_chanel,
            node_manager_password: &*node_manager_password.leak(),
        })))
    }
}

impl Deref for AppState {
    type Target = InnerAppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
