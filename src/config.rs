use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Solana RPC endpoint
    pub solana_rpc_url: String,
    
    /// gRPC server address
    pub grpc_address: String,
    
    /// Target regions for optimized connectivity
    pub target_regions: Vec<String>,
    
    /// Authentication keypair path
    pub keypair_path: Option<String>,
    
    /// Log level
    pub log_level: String,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    
    /// Maximum concurrent connections
    pub max_connections: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            grpc_address: "127.0.0.1:50051".to_string(),
            target_regions: vec!["us-east-1".to_string(), "us-west-1".to_string()],
            keypair_path: None,
            log_level: "info".to_string(),
            connection_timeout: 30,
            max_connections: 1000,
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            let config_str = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&config_str)?;
            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Config::default())
        }
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }
}
