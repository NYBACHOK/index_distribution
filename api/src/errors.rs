use axum::{
    Json,
    extract::rejection::{BytesRejection, JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Offset for client errors explicit codes
pub const CLIENT_ERROR_CODES_OFFSET: u16 = 4000;
/// Offset for server errors explicit codes
pub const SERVER_ERROR_CODES_OFFSET: u16 = 5000;

/// Response with additional information about error
#[derive(Debug, serde::Serialize, utoipa::ToResponse, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// Error message with additional info
    pub message: String,
    /// Code for troubleshooting
    pub code: u16,
}

/// All possible lib errors
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum RouteError {
    /// The request body contained invalid JSON
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    /// The request body contained invalid Bytes
    #[error(transparent)]
    BytesRejection(#[from] BytesRejection),
    /// Invalid request by other validation. Like inside of route
    #[error("Invalid request. Reason: {0}")]
    Rejection(String),
    /// Missing header
    #[error("Missing `{0}` header")]
    MissingHeader(&'static str),

    /// Error in middleware
    #[error("Error in request pipeline: {0}")]
    Middleware(String),

    /// Failed to find smth
    #[error("Failed to find {0}")]
    NotFound(&'static str),

    #[error("Failed to run query. Reason: {0}")]
    Db(#[from] sqlx::Error),

    #[error(" {0}")]
    S3(#[from] s3::error::S3Error),

    #[error("Failed connection to cache. Reason: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Failed communication with node. Reason: {0}")]
    Reqwest(#[from] reqwest::Error),

    // IO errors
    /// Failed IO
    #[error("Failed IO. Description: {desc}. Err: {err}")]
    Io {
        /// Inner error
        err: std::io::Error,
        /// Additional desc
        desc: String,
    },

    /// For different client errors
    #[error("Invalid request. Reason: {0}")]
    InvalidRequest(String),
    /// For errors that should never occur, but happened somehow
    #[error("Unexpected server error. Reason: {0}")]
    Unexpected(String),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl IntoResponse for RouteError {
    // TODO: Macros for this?
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            RouteError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (
                    StatusCode::BAD_REQUEST,
                    CLIENT_ERROR_CODES_OFFSET + 1,
                    rejection.body_text(),
                )
            }
            RouteError::BytesRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (
                    StatusCode::BAD_REQUEST,
                    CLIENT_ERROR_CODES_OFFSET + 2,
                    rejection.body_text(),
                )
            }
            RouteError::Rejection(_) => (
                StatusCode::BAD_REQUEST,
                CLIENT_ERROR_CODES_OFFSET + 3,
                self.to_string(),
            ),
            RouteError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                CLIENT_ERROR_CODES_OFFSET + 4,
                self.to_string(),
            ),
            Self::InvalidRequest(_) => (
                StatusCode::BAD_REQUEST,
                SERVER_ERROR_CODES_OFFSET + 41,
                self.to_string(),
            ),
            Self::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR_CODES_OFFSET + 51,
                self.to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR_CODES_OFFSET + 50,
                self.to_string(),
            ),
        };

        if matches!(self, RouteError::Unexpected(_)) {
            tracing::error!("Code: {code} | Msg: {message}");
        }

        (status, Json::from(ErrorResponse { message, code })).into_response()
    }
}
