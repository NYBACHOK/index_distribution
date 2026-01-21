use axum::extract::State;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    core::types::BundleKind,
    errors::RouteError,
    state::AppState,
    utils::{json_extractor::Json, jwt_auth::UserCredentials},
};

#[derive(serde::Deserialize)]
pub struct CreateBundleRequest {
    kind: BundleKind,
}

#[derive(serde::Serialize)]
pub struct CreateBundleResponse {
    id: Uuid,
}

#[derive(FromRow)]
struct Record {
    id: Uuid,
}

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
