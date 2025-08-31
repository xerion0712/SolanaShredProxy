use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber;

mod config;
mod proxy;
mod grpc;
mod shred_stream;

#[derive(Parser)]
#[command(name = "solanashredproxy")]
#[command(about = "Solana ShredProxy - Ultra-low latency access to leader shreds")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    
    info!("Starting Solana ShredProxy...");
    
    // Load configuration
    let config = config::Config::load(&cli.config)?;
    info!("Configuration loaded from {}", cli.config);
    
    // Initialize proxy
    let proxy = proxy::ShredProxy::new(config).await?;
    info!("ShredProxy initialized successfully");
    
    // Start the proxy service
    if let Err(e) = proxy.run().await {
        error!("Proxy service failed: {}", e);
        return Err(e.into());
    }
    
    Ok(())
}
