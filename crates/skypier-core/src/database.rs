use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{Vector, SearchResult, DatabaseStats, DistanceMetric};
use skypier_storage::Storage;
use skypier_index::VectorIndex;

pub struct VectorDatabase {
    storage: Arc<dyn Storage>,
    index: Arc<RwLock<dyn VectorIndex>>,
    distance_metric: DistanceMetric,
    dimensions: Option<usize>,
}

impl VectorDatabase {
    pub async fn new(data_dir: &str) -> Result<Self> {
        let storage = Arc::new(skypier_storage::RedbStorage::new(data_dir).await?);
        let index = Arc::new(RwLock::new(skypier_index::HnswIndex::new(768)?));
        
        Ok(Self {
            storage,
            index,
            distance_metric: DistanceMetric::Cosine,
            dimensions: None,
        })
    }

    pub async fn insert_vectors(&self, vectors: Vec<Vector>) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        let mut index = self.index.write().await;

        for vector in vectors {
            // Validate dimensions
            if let Some(dims) = self.dimensions {
                if vector.data.len() != dims {
                    return Err(anyhow!("Vector dimension mismatch: expected {}, got {}", dims, vector.data.len()));
                }
            }

            // Store vector in persistent storage
            self.storage.store_vector(&vector).await?;
            
            // Add to index
            index.add_vector(&vector.id, &vector.data)?;
            
            ids.push(vector.id);
        }

        Ok(ids)
    }

    pub async fn get_vector(&self, id: &str) -> Result<Option<Vector>> {
        self.storage.get_vector(id).await
    }

    pub async fn search(&self, query: &[f32], k: usize, threshold: f32) -> Result<Vec<SearchResult>> {
        let index = self.index.read().await;
        let candidates = index.search(query, k * 2)?; // Get more candidates for reranking
        
        let mut results = Vec::new();
        
        for candidate in candidates {
            if candidate.score >= threshold {
                if let Some(vector) = self.storage.get_vector(&candidate.id).await? {
                    results.push(SearchResult {
                        id: candidate.id,
                        score: candidate.score,
                        metadata: vector.metadata,
                    });
                }
            }
            
            if results.len() >= k {
                break;
            }
        }

        Ok(results)
    }

    pub async fn search_in_collection(
        &self,
        collection: &str,
        query: &[f32],
        k: usize,
        threshold: f32,
    ) -> Result<Vec<SearchResult>> {
        let index = self.index.read().await;
        let candidates = index.search(query, k * 5)?; // Get more candidates for filtering
        
        let mut results = Vec::new();
        
        for candidate in candidates {
            if candidate.score >= threshold {
                if let Some(vector) = self.storage.get_vector(&candidate.id).await? {
                    if vector.collection.as_ref().map(|s| s.as_str()) == Some(collection) {
                        results.push(SearchResult {
                            id: candidate.id,
                            score: candidate.score,
                            metadata: vector.metadata,
                        });
                    }
                }
            }
            
            if results.len() >= k {
                break;
            }
        }

        Ok(results)
    }

    pub async fn delete_vector(&self, id: &str) -> Result<bool> {
        let removed = self.storage.delete_vector(id).await?;
        if removed {
            let mut index = self.index.write().await;
            index.remove_vector(id)?;
        }
        Ok(removed)
    }

    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let total_vectors = self.storage.count_vectors().await?;
        let storage_size = self.storage.size_bytes().await?;
        
        Ok(DatabaseStats {
            total_vectors,
            dimensions: self.dimensions.unwrap_or(0),
            storage_size_bytes: storage_size,
        })
    }

    pub async fn compact(&self) -> Result<()> {
        self.storage.compact().await?;
        Ok(())
    }

    pub async fn backup(&self, backup_path: &str) -> Result<()> {
        self.storage.backup(backup_path).await?;
        Ok(())
    }
}
