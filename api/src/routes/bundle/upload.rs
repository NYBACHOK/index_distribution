use axum::extract::{Query, State};

use crate::{
    core::upload::upload_archive_for_bundle,
    errors::RouteError,
    routes::bundle::BundleQuery,
    state::AppState,
    utils::{jwt_auth::UserCredentials, zip_file::ZipFile},
};

pub async fn upload(
    user: UserCredentials,
    Query(BundleQuery { bundle_id }): Query<BundleQuery>,
    State(state): State<AppState>,
    archive: ZipFile,
) -> Result<(), RouteError> {
    upload_archive_for_bundle(&state.bucket, &state.pool, archive, bundle_id, &user.user).await?;

    Ok(())
}
