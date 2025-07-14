use anyhow::Result;
use std::collections::HashMap;

use crate::{VectorIndex, SearchResult};

pub struct FlatIndex {
    vectors: HashMap<String, Vec<f32>>,
}

impl FlatIndex {
    pub fn new() -> Self {
        Self {
            vectors: HashMap::new(),
        }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

impl VectorIndex for FlatIndex {
    fn add_vector(&mut self, id: &str, vector: &[f32]) -> Result<()> {
        self.vectors.insert(id.to_string(), vector.to_vec());
        Ok(())
    }

    fn remove_vector(&mut self, id: &str) -> Result<bool> {
        Ok(self.vectors.remove(id).is_some())
    }

    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>> {
        let mut results: Vec<_> = self
            .vectors
            .iter()
            .map(|(id, vector)| {
                let score = Self::cosine_similarity(query, vector);
                SearchResult {
                    id: id.clone(),
                    score,
                }
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Take top k
        results.truncate(k);
        
        Ok(results)
    }

    fn size(&self) -> usize {
        self.vectors.len()
    }

    fn clear(&mut self) {
        self.vectors.clear();
    }
}
