use axum::extract::State;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    core::types::BundleKind,
    errors::RouteError,
    state::AppState,
    utils::{json_extractor::Json, jwt_auth::UserCredentials},
};

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct CreateBundleRequest {
    kind: BundleKind,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct CreateBundleResponse {
    #[serde(with = "crate::utils::serde::uuid_as_base64")]
    id: Uuid,
}

#[derive(FromRow)]
struct Record {
    id: Uuid,
}

#[utoipa::path(
    put,
    path = "/bundle/create",
    request_body = crate::routes::bundle::CreateBundleRequest,
    responses(
        (status = 200, description = "Bundle created", body = crate::routes::bundle::CreateBundleResponse),
        (status = 400, description = "Bad request", body = crate::errors::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
        (status = 500, description = "Server error", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
    Json(CreateBundleRequest { kind }): Json<CreateBundleRequest>,
) -> Result<axum::Json<CreateBundleResponse>, RouteError> {
    let mut transaction = state.pool.begin().await?;

    let record: Record =
        sqlx::query_as("insert into bundles (owner, kind) values ($1, $2) returning id;")
            .bind(&user.user_id)
            .bind(kind.as_ref())
            .fetch_one(&mut *transaction)
            .await?;

    transaction.commit().await?;

    Ok(axum::Json(CreateBundleResponse { id: record.id }))
}
