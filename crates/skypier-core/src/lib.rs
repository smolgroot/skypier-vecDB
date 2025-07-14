use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

pub mod database;
pub mod similarity;

pub use database::VectorDatabase;
pub use skypier_storage::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_vectors: usize,
    pub dimensions: usize,
    pub storage_size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

impl DistanceMetric {
    pub fn compute(&self, a: &[f32], b: &[f32]) -> Result<f32> {
        if a.len() != b.len() {
            return Err(anyhow!("Vector dimensions must match"));
        }

        match self {
            DistanceMetric::Cosine => similarity::cosine_similarity(a, b),
            DistanceMetric::Euclidean => similarity::euclidean_distance(a, b),
            DistanceMetric::DotProduct => similarity::dot_product(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
