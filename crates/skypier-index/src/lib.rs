use anyhow::Result;

pub mod flat;
pub mod hnsw;

pub use flat::FlatIndex;
pub use hnsw::HnswIndex;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
}

pub trait VectorIndex: Send + Sync {
    fn add_vector(&mut self, id: &str, vector: &[f32]) -> Result<()>;
    fn remove_vector(&mut self, id: &str) -> Result<bool>;
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>>;
    fn size(&self) -> usize;
    fn clear(&mut self);
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
