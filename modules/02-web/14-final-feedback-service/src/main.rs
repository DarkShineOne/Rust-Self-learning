#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("info,web_14_final_feedback=info")
            }),
        )
        .init();

    if let Err(error) = web_14_final_feedback::run().await {
        tracing::error!(%error, "feedback service terminated");
        std::process::exit(1);
    }
}
