use anyhow::Result;
use clap::{Arg, Command};
use skypier_core::VectorDatabase;
use skypier_network::P2PNode;
use std::sync::Arc;
use tokio;
use tracing::{info, warn};

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let matches = Command::new("SkyPier VecDB")
        .version("0.1.0")
        .author("SkyPier Team")
        .about("A decentralized minimal vector database for AI infrastructure")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .default_value("config.toml"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets the HTTP server port")
                .default_value("8080"),
        )
        .arg(
            Arg::new("p2p-port")
                .long("p2p-port")
                .value_name("PORT")
                .help("Sets the P2P network port")
                .default_value("7777"),
        )
        .get_matches();

    let config_file = matches.get_one::<String>("config").unwrap().clone();
    let http_port = matches.get_one::<String>("port").unwrap().clone();
    let p2p_port = matches.get_one::<String>("p2p-port").unwrap().clone();

    info!("Starting SkyPier VecDB");
    info!("Config file: {}", config_file);
    info!("HTTP port: {}", http_port);
    info!("P2P port: {}", p2p_port);

    // Initialize the vector database
    let db = Arc::new(VectorDatabase::new("./data").await?);

    // Initialize P2P networking
    let network_config = skypier_network::NetworkConfig {
        port: p2p_port.parse()?,
        bootstrap_peers: vec![],
        max_peers: 50,
    };
    let mut p2p_node = P2PNode::new(network_config).await?;
    let p2p_handle = tokio::spawn(async move {
        if let Err(e) = p2p_node.start().await {
            warn!("P2P node error: {}", e);
        }
    });

    // Start HTTP API server
    let api_handle = tokio::spawn({
        let db = Arc::clone(&db);
        async move {
            if let Err(e) = api::start_server(db, http_port.parse().unwrap()).await {
                warn!("API server error: {}", e);
            }
        }
    });

    // Wait for both services
    tokio::select! {
        _ = p2p_handle => {
            info!("P2P node terminated");
        }
        _ = api_handle => {
            info!("API server terminated");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down");
        }
    }

    Ok(())
}
