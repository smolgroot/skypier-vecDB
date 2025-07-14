use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod redb_storage;

pub use redb_storage::RedbStorage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    pub id: String,
    pub data: Vec<f32>,
    pub metadata: Option<HashMap<String, String>>,
    pub collection: Option<String>,
    pub created_at: u64,
}

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    async fn store_vector(&self, vector: &Vector) -> Result<()>;
    async fn get_vector(&self, id: &str) -> Result<Option<Vector>>;
    async fn delete_vector(&self, id: &str) -> Result<bool>;
    async fn count_vectors(&self) -> Result<usize>;
    async fn size_bytes(&self) -> Result<usize>;
    async fn compact(&self) -> Result<()>;
    async fn backup(&self, backup_path: &str) -> Result<()>;
    async fn list_collections(&self) -> Result<Vec<String>>;
    async fn get_vectors_in_collection(&self, collection: &str) -> Result<Vec<Vector>>;
}
