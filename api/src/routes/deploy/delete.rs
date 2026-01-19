use axum::extract::{Query, State};

use crate::{
    errors::RouteError, routes::UuidQuery, state::AppState, utils::jwt_auth::UserCredentials,
};

pub async fn delete(
    user: UserCredentials,
    State(state): State<AppState>,
    Query(UuidQuery { id }): Query<UuidQuery>,
) -> Result<(), RouteError> {
    let mut transaction = state.pool.begin().await?;

    sqlx::query("update bundles set is_deployed = true where id == $1 and owner == $2")
        .bind(id)
        .bind(user.user)
        .execute(&mut *transaction)
        .await?;

    // TODO: delete deployment from app

    Ok(())
}
