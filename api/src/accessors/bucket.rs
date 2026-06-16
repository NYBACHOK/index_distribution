use std::str::FromStr;

/// S3 errors
#[derive(Debug, thiserror::Error)]
pub enum S3Errors {
    /// Custom region parse
    #[error(
        "Failed to load aws region from env. Set `AWS_REGION` or `AWS_ENDPOINT` to proper values. Reason: {0}"
    )]
    Region(#[from] s3::region::error::RegionError),
    /// Not found region
    #[error("Failed to parse region")]
    RegionParse(#[from] std::str::Utf8Error),
    /// Failed to parse credentials
    #[error("Failed to load credentials. Reason: {0}")]
    Credentials(#[from] s3::creds::error::CredentialsError),
    /// Inner error
    #[error("{0}")]
    S3(#[from] s3::error::S3Error),
}

/// Config for client
#[derive(Debug)]
pub struct AwsClientConfig {
    /// Custom endpoint
    pub endpoint: Option<String>,
    /// Custom region
    pub region: String,
}

pub async fn setup_s3(
    bucket_name: impl AsRef<str>,
    should_create: bool,
    cfg: Option<AwsClientConfig>,
) -> Result<Box<s3::Bucket>, S3Errors> {
    let region = match cfg {
        Some(AwsClientConfig {
            endpoint: Some(endpoint),
            region,
        }) => s3::Region::Custom { region, endpoint },
        Some(AwsClientConfig {
            endpoint: None,
            region,
        }) => s3::Region::from_str(&region)?,
        None => s3::Region::from_default_env()?,
    };

    let credentials = s3::creds::Credentials::from_env()?;

    let mut bucket = s3::Bucket::new(bucket_name.as_ref(), region.clone(), credentials.clone())?
        .with_path_style();

    let is_exists = bucket.exists().await?;

    if !is_exists && should_create {
        tracing::info!(
            "Bucket `{}` don't exists. Creating new",
            bucket_name.as_ref()
        );

        bucket = s3::Bucket::create_with_path_style(
            bucket_name.as_ref(),
            region,
            credentials,
            s3::BucketConfiguration::private(),
        )
        .await?
        .bucket;
    } else if !is_exists {
        tracing::warn!("Bucket `{}` don't exists", bucket_name.as_ref())
    }

    Ok(bucket)
}
