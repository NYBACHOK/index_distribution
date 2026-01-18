use axum::http::HeaderMap;

#[derive(Debug, Clone)]
pub struct Json<T>(pub T);

impl<S, T> axum::extract::FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned,
{
    type Rejection = crate::errors::RouteError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let res = if json_content_type(req.headers()) {
            let bytes = axum::body::Bytes::from_request(req, state).await?;

            let axum::Json(res) = axum::Json::<T>::from_bytes(&bytes)?;

            res
        } else {
            return Err(crate::errors::RouteError::MissingHeader("Content-Type"));
        };

        Ok(Self(res))
    }
}

pub fn json_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(axum::http::header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"));

    is_json_content_type
}
