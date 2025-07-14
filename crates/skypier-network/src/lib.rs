use anyhow::Result;

pub mod consensus;
pub mod p2p_node;
pub mod replication;

pub use consensus::ConsensusEngine;
pub use p2p_node::P2PNode;
pub use replication::ReplicationManager;

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub port: u16,
    pub bootstrap_peers: Vec<String>,
    pub max_peers: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            bootstrap_peers: vec![],
            max_peers: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
