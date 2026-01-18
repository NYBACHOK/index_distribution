use std::net::SocketAddr;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let _ = dotenv::dotenv();

    let Args {
        host, log_level, ..
    } = Args::parse();

    setup_logger(log_level);

    api::start_api(host).await?;

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
