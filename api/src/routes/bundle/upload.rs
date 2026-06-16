use axum::extract::{Query, State};

use crate::{
    errors::RouteError,
    routes::UuidQuery,
    state::AppState,
    utils::{jwt_auth::UserCredentials, zip_file::ZipFile},
};

#[utoipa::path(
    post,
    path = "/bundle/upload",
    params(("id" = uuid::Uuid, Query, description = "Bundle id")),
    responses(
        (status = 200, description = "Uploaded"),
        (status = 400, description = "Bad request", body = crate::errors::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn upload(
    user: UserCredentials,
    Query(UuidQuery { id }): Query<UuidQuery>,
    State(state): State<AppState>,
    archive: ZipFile,
) -> Result<(), RouteError> {
    let mut transaction = state.pool.begin().await?;

    state
        .bucket
        .put_object_stream(
            &mut archive.0.into_inner().into_inner().as_slice(),
            format!("{id}.zip"),
        )
        .await?;

    sqlx::query("update bundles set is_uploaded = true where id = $1 and owner = $2")
        .bind(id)
        .bind(user.user_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}
