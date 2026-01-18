use std::io::Cursor;

use axum::{RequestExt, body::Bytes, http::header::CONTENT_TYPE};
use headers::ContentType;
use zip::ZipArchive;

use crate::errors::RouteError;

pub const FILE_LIMIT: usize = 100_000_000; // 100mb more than enough for static website...

#[derive(Debug)]
pub struct ZipFile {
    file: ZipArchive<Cursor<Bytes>>,
}

impl<S> axum::extract::FromRequest<S> for ZipFile
where
    S: Send + Sync,
{
    type Rejection = crate::errors::RouteError;

    async fn from_request(
        mut req: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum_extra::TypedHeader(content_type) = req
            .extract_parts::<axum_extra::TypedHeader<ContentType>>()
            .await
            .map_err(|_e| RouteError::MissingHeader(CONTENT_TYPE.as_str()))?;

        if content_type.to_string() != "application/zip" {
            return Err(RouteError::Rejection(
                "only `application/zip` is supported".to_owned(),
            ));
        }

        let body = axum::body::to_bytes(req.into_body(), FILE_LIMIT)
            .await
            .map_err(|e| {
                RouteError::Rejection(format!("failed to extract request body. Error: {e}"))
            })?;

        let cursor = Cursor::new(body);

        let zip = ZipArchive::new(cursor)
            .map_err(|e| RouteError::Rejection(format!("invalid archive. Error: {e}")))?;

        if zip.is_empty() {
            return Err(RouteError::Rejection("empty archive".to_string()));
        }

        Ok(Self { file: zip })
    }
}
