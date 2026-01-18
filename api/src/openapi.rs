use crate::errors::ErrorResponse;

#[derive(utoipa::OpenApi)]
#[openapi(paths(), components(schemas(), responses(ErrorResponse)))]
pub struct ApiDoc;
