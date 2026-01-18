use axum::{RequestExt, http::header::CONTENT_TYPE};
use headers::ContentType;

use crate::errors::RouteError;

pub struct ZipFile {}

impl<S> axum::extract::FromRequest<S> for ZipFile
where
    S: Send + Sync,
{
    type Rejection = crate::errors::RouteError;

    async fn from_request(
        req: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum_extra::TypedHeader(content_type) = req
            .extract::<axum_extra::TypedHeader<ContentType>, _>()
            .await
            .map_err(|_e| RouteError::MissingHeader(CONTENT_TYPE.as_str()))?;

        if content_type.to_string() != "application/zip" {
            return Err(RouteError::Rejection(
                "only `application/zip` is supported".to_owned(),
            ));
        }

        todo!()
    }
}
