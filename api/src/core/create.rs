use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{core::types::BundleKind, utils::jwt_auth::UserCredentials};

#[derive(FromRow)]
struct Record {
    id: Uuid,
}

pub async fn create_bundle_record(
    user: &UserCredentials,
    kind: BundleKind,
    pool: &sqlx::PgPool,
) -> Result<uuid::Uuid, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let record: Record =
        sqlx::query_as("insert into bundles (owner, kind) values ($1, $2) returning id;")
            .bind(&user.user)
            .bind(kind.as_ref())
            .fetch_one(&mut *transaction)
            .await?;

    transaction.commit().await?;

    Ok(record.id)
}
