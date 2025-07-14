use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
            id: Uuid::new_v4().to_string(),
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
