use {
    clap::Parser,
    moqt::PublisherConfig,
    snafu::{ResultExt, Whatever},
    std::path::PathBuf,
    tracing::Level,
};

/// MOQT Relay
#[derive(Debug, Parser)]
struct Cli {
    /// Log level
    #[arg(short = 'l', long = "log", default_value = "info")]
    log_level: Level,

    #[command(flatten)]
    config: PublisherConfig,
}

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let cli = Cli::parse();

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(cli.log_level)
            .finish(),
    )
    .expect("setting tracing default failed");

    let publisher = moqt::Publisher::new(cli.config)
        .await
        .whatever_context("failed to build publisher")?;

    Ok(())
}
