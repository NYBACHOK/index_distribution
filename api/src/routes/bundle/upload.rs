use axum::extract::State;

use crate::{
    errors::RouteError,
    state::AppState,
    utils::{jwt_auth::UserCredentials, zip_file::ZipFile},
};

pub async fn upload(
    user: UserCredentials,
    State(state): State<AppState>,
    archive: ZipFile,
) -> Result<(), RouteError> {
    Ok(())
}
