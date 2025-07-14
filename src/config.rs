use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub p2p: P2PConfig,
    pub storage: StorageConfig,
    pub index: IndexConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct P2PConfig {
    pub port: u16,
    pub bootstrap_peers: Vec<String>,
    pub max_peers: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    pub data_dir: String,
    pub max_file_size: usize,
    pub compression: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexConfig {
    pub index_type: String, // "faiss" or "embedded"
    pub dimensions: usize,
    pub distance_metric: String, // "cosine", "euclidean", "dot_product"
    pub ef_construction: usize,
    pub ef_search: usize,
    pub max_connections: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            p2p: P2PConfig {
                port: 7777,
                bootstrap_peers: vec![],
                max_peers: 50,
            },
            storage: StorageConfig {
                data_dir: "./data".to_string(),
                max_file_size: 1024 * 1024 * 1024, // 1GB
                compression: true,
            },
            index: IndexConfig {
                index_type: "embedded".to_string(),
                dimensions: 768,
                distance_metric: "cosine".to_string(),
                ef_construction: 200,
                ef_search: 50,
                max_connections: 16,
            },
        }
    }
}
