use std::net::SocketAddr;

use api::AwsClientConfig;
use clap::Parser;

#[derive(Debug, Clone, clap::Parser)]
#[non_exhaustive]
struct Args {
    /// Host to server the server
    #[ arg( long, required = false,  env = "SERVE_AT", default_value_t = SocketAddr::from( ( [ 127,0,0,1 ], 5000 ) ) ) ]
    host: SocketAddr,

    /// Verbosity of logs
    #[ arg( long, required = false, default_value_t = tracing::Level::INFO, env = "LOG_LEVEL" ) ]
    log_level: tracing::Level,

    /// Bucket where data is stored
    #[arg(short, long, env = "S3_BUCKET", required = false)]
    bucket: String,
    /// Should create bucket if not exists
    #[arg(long, required = false, default_value_t = false)]
    create_bucket: bool,
    /// Custom endpoint to S3
    #[arg(long, required = false, env = "AWS_ENDPOINT")]
    aws_endpoint: Option<String>,
    /// AWS Region
    #[arg(long, required = false, env = "AWS_REGION")]
    region: Option<String>,

    /// Content of RSA public key in base64 format
    #[arg(long, env = "RSA_PUB_KEY_BASE64", required = true)]
    rsa_pub_key_base64: String,

    /// Connection string to postgres database
    #[arg(long, required = false, env = "DB_CONNECTION_STRING")]
    connection_string: String,

    /// Connection string to cache instance
    #[arg(long, required = false, default_value_t = String::from("valkey://localhost:6379"), env = "REDIS_CONNECTION_STRING")]
    redis_connection_string: String,

    /// Password for node manager for managing nodes
    #[arg(long, required = true, env = "NODE_MANAGER_PASSWORD")]
    node_manager_password: String,

    /// Password for accessing nodes(apps)
    #[arg(long, required = true, env = "APP_PASSWORD")]
    app_password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let _ = dotenv::dotenv();

    let Args {
        host,
        log_level,
        aws_endpoint,
        region,
        bucket,
        create_bucket,
        rsa_pub_key_base64,
        connection_string,
        redis_connection_string,
        node_manager_password,
        app_password,
        ..
    } = Args::parse();

    setup_logger(log_level);

    let aws_clonfig = match (aws_endpoint, region) {
        (endpoint, Some(region)) => Some(AwsClientConfig { endpoint, region }),
        _ => None,
    };

    api::start_api(
        host,
        &bucket,
        create_bucket,
        aws_clonfig,
        rsa_pub_key_base64,
        connection_string,
        redis_connection_string,
        node_manager_password,
        app_password,
    )
    .await?;

    Ok(())
}

fn setup_logger(level: tracing::Level) {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::from_default_env()
        .add_directive(level.into())
        .add_directive("hyper=warn".parse().expect("valid directive"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
}
