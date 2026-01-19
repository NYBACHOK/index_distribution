use uuid::Uuid;

use crate::{errors::RouteError, utils::zip_file::ZipFile};

pub async fn upload_archive_for_bundle(
    bucket: &s3::Bucket,
    pool: &sqlx::PgPool,
    archive: ZipFile,
    id: Uuid,
    owner: &str,
) -> Result<(), RouteError> {
    let mut transaction = pool.begin().await?;

    bucket
        .put_object_stream(&mut archive.0.into_inner(), format!("{id}.zip"))
        .await?;

    sqlx::query("update bundles set is_uploaded = true where id == $1 and owner == $2")
        .bind(id)
        .bind(owner)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}
