use axum::extract::State;

use crate::{errors::RouteError, state::AppState, utils::jwt_auth::UserCredentials};

pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
) -> Result<(), RouteError> {
    Ok(())
}
