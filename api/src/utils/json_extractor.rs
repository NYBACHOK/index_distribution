use axum::{RequestExt, http::header::CONTENT_TYPE};
use headers::ContentType;

use crate::errors::RouteError;

#[derive(Debug, Clone)]
pub struct Json<T>(pub T);

impl<S, T> axum::extract::FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned,
{
    type Rejection = RouteError;

    async fn from_request(
        mut req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum_extra::TypedHeader(content_type) = req
            .extract_parts::<axum_extra::TypedHeader<ContentType>>()
            .await
            .map_err(|_e| RouteError::MissingHeader(CONTENT_TYPE.as_str()))?;

        let res = if mime::Mime::from(content_type) == mime::APPLICATION_JSON {
            let bytes = axum::body::Bytes::from_request(req, state).await?;

            let axum::Json(res) = axum::Json::<T>::from_bytes(&bytes)?;

            res
        } else {
            return Err(RouteError::MissingHeader("Content-Type"));
        };

        Ok(Self(res))
    }
}
