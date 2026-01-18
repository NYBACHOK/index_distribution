use axum::extract::State;
use uuid::Uuid;

use crate::{
    core::{create::create_bundle_record, types::BundleKind},
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

pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
    Json(CreateBundleRequest { kind }): Json<CreateBundleRequest>,
) -> Result<axum::Json<CreateBundleResponse>, RouteError> {
    let id = create_bundle_record(&user, kind, &state.pool).await?;

    Ok(axum::Json(CreateBundleResponse { id }))
}
