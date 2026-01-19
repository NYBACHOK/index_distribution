use axum::extract::{Query, State};

use crate::{
    core::upload::upload_archive_for_bundle,
    errors::RouteError,
    routes::UuidQuery,
    state::AppState,
    utils::{jwt_auth::UserCredentials, zip_file::ZipFile},
};

pub async fn upload(
    user: UserCredentials,
    Query(UuidQuery { id }): Query<UuidQuery>,
    State(state): State<AppState>,
    archive: ZipFile,
) -> Result<(), RouteError> {
    upload_archive_for_bundle(&state.bucket, &state.pool, archive, id, &user.user).await?;

    Ok(())
}
