mod connect;
mod disconnect;

use axum::{extract::FromRequestParts, http::StatusCode};
use axum_extra::extract::CookieJar;

use crate::state::AppState;

pub use self::{connect::*, disconnect::*};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum NodeKind {
    Mobile,
    Web,
    Dekstop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeManager;

impl FromRequestParts<AppState> for NodeManager {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("Infallible error");

        let password: Option<String> = jar
            .get("nodeManagerPassword")
            .map(|cookie| cookie.value().to_owned());

        let password = match password {
            Some(password) => password,
            None => parts
                .headers
                .get("Node-Manager-Password")
                .map(|this| this.to_str().map(|this| this.to_owned()).ok())
                .flatten()
                .ok_or(StatusCode::UNAUTHORIZED)?,
        };

        if !state.is_password_matches(password) {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(Self)
    }
}
