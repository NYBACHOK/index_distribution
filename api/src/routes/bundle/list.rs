use axum::extract::State;

use crate::{
    core::types::BundleKind, errors::RouteError, state::AppState, utils::jwt_auth::UserCredentials,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Bundle {
    pub is_uploaded: bool,
    pub is_deployed: bool,
    pub kind: BundleKind,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ListResponse {
    items: Vec<Bundle>,
}

#[utoipa::path(
    get,
    path = "/bundle/list",
    responses(
        (status = 200, description = "List bundles", body = crate::routes::bundle::ListResponse),
        (status = 401, description = "Unauthorized", body = crate::errors::ErrorResponse),
    ),
)]
pub async fn list(
    user: UserCredentials,
    State(state): State<AppState>,
) -> Result<axum::Json<ListResponse>, RouteError> {
    let items: Vec<Bundle> = sqlx::query_as(
        "select b.is_uploaded, b.is_deployed, b.kind from bundles b where b.owner == $1",
    )
    .bind(user.user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(axum::Json(ListResponse { items }))
}
