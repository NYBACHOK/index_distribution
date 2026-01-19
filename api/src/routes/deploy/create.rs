use axum::extract::State;
use uuid::Uuid;

use crate::{
    errors::RouteError,
    state::AppState,
    utils::{json_extractor::Json, jwt_auth::UserCredentials},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DeployCreate {
    pub bundle_id: Uuid,
    pub node_id: Uuid,
}

pub async fn create(
    user: UserCredentials,
    State(state): State<AppState>,
    Json(DeployCreate { bundle_id, node_id }): Json<DeployCreate>,
) -> Result<(), RouteError> {
    

    Ok(())
}
