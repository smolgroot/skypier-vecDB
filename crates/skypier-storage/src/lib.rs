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

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            data,
            metadata: None,
            collection: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_id(id: String, data: Vec<f32>) -> Self {
        Self {
            id,
            data,
            metadata: None,
            collection: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_collection(mut self, collection: String) -> Self {
        self.collection = Some(collection);
        self
    }

    pub fn dimensions(&self) -> usize {
        self.data.len()
    }

    pub fn normalize(&mut self) {
        let norm = self.data.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut self.data {
                *x /= norm;
            }
        }
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
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
    async fn get_first_vector(&self) -> Result<Option<Vector>>;
}
