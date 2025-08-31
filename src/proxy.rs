use anyhow::Result;
use crate::config::Config;
use crate::grpc::GrpcServer;
use crate::shred_stream::ShredStream;
use tracing::{info, error, warn};

pub struct ShredProxy {
    config: Config,
    shred_stream: ShredStream,
    grpc_server: GrpcServer,
}

impl ShredProxy {
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing ShredProxy...");
        
        // Initialize shred stream
        let shred_stream = ShredStream::new(&config).await?;
        info!("ShredStream initialized");
        
        // Initialize gRPC server
        let grpc_server = GrpcServer::new(&config).await?;
        info!("gRPC server initialized");
        
        Ok(Self {
            config,
            shred_stream,
            grpc_server,
        })
    }
    
    pub async fn run(&self) -> Result<()> {
        info!("Starting ShredProxy service...");
        
        // Start shred stream processing
        let shred_handle = {
            let shred_stream = self.shred_stream.clone();
            tokio::spawn(async move {
                if let Err(e) = shred_stream.run().await {
                    error!("ShredStream failed: {}", e);
                }
            })
        };
        
        // Start gRPC server
        let grpc_handle = {
            let grpc_server = self.grpc_server.clone();
            tokio::spawn(async move {
                if let Err(e) = grpc_server.run().await {
                    error!("gRPC server failed: {}", e);
                }
            })
        };
        
        // Wait for both services
        tokio::select! {
            result = shred_handle => {
                if let Err(e) = result {
                    error!("ShredStream task failed: {}", e);
                }
            }
            result = grpc_handle => {
                if let Err(e) = result {
                    error!("gRPC server task failed: {}", e);
                }
            }
        }
        
        warn!("ShredProxy service stopped");
        Ok(())
    }
}
