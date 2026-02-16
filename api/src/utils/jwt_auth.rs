use std::sync::LazyLock;

use axum::{RequestPartsExt, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use headers::{Authorization, authorization::Bearer};

use crate::{
    errors::{CLIENT_ERROR_CODES_OFFSET, ErrorResponse},
    state::AppState,
};

/// Storage for keys
#[derive(Clone)]
pub struct JwtKeys {
    /// Decode key
    pub decoding: jsonwebtoken::DecodingKey,
}

impl JwtKeys {
    /// Load from pem encoded rsa keys
    pub fn try_from_pem(public: impl AsRef<[u8]>) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self {
            decoding: jsonwebtoken::DecodingKey::from_rsa_pem(public.as_ref())?,
        })
    }
}

impl std::fmt::Debug for JwtKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtKeys")
            .field("decoding", &"SECRET")
            .finish()
    }
}

/// Extracted claims
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserCredentials {
    // /// The issuer of the JWT
    // pub iss: String,
    // /// The expiration time on or after which the JWT MUST NOT be accepted for processing
    // pub exp: usize,
    // /// The time at which the JWT was issued
    // pub iat: usize,
    //   /// Audience
    //   pub aud: String,
    pub user_id: String,
}

impl axum::extract::FromRequestParts<AppState> for UserCredentials {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if cfg!(debug_assertions) {
            return Ok(UserCredentials {
                user_id: "DEBUG_USER".to_owned(),
            });
        }

        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("Infallible error");

        let access_token: Option<String> = jar
            .get("accessToken")
            .map(|cookie| cookie.value().to_owned());

        let token = match access_token {
            Some(token) => token,
            None => {
                let axum_extra::TypedHeader(Authorization(bearer)) = parts
                    .extract::<axum_extra::TypedHeader<Authorization<Bearer>>>()
                    .await?;

                bearer.token().to_owned()
            }
        };

        static VALIDATION_OPT: LazyLock<jsonwebtoken::Validation> = LazyLock::new(|| {
            let opt = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

            opt
        });

        let token_data =
            jsonwebtoken::decode::<UserCredentials>(&token, &state.keys.decoding, &VALIDATION_OPT)?;

        Ok(token_data.claims)
    }
}

/// Errors related to jwt
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Invalid token
    #[error("Invalid token. Reason: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    /// Failed to read token from headers
    #[error("Failed to read token from header. Reason: {0}")]
    Extraction(#[from] axum_extra::typed_header::TypedHeaderRejection),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, offset) = match self {
            AuthError::Extraction { .. } => (StatusCode::UNAUTHORIZED, 401),
            AuthError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, 402),
        };

        (
            status,
            axum::Json::from(ErrorResponse {
                message: self.to_string(),
                code: CLIENT_ERROR_CODES_OFFSET + offset,
            }),
        )
            .into_response()
    }
}
