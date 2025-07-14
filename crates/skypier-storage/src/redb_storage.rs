use anyhow::Result;
use redb::{Database, TableDefinition, ReadableTable, ReadableTableMetadata};
use serde_json;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use tokio::task;

use crate::{Storage, Vector};

const VECTORS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("vectors");
const METADATA_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("metadata");

pub struct RedbStorage {
    db: Arc<Database>,
    data_dir: String,
}

impl RedbStorage {
    pub async fn new(data_dir: &str) -> Result<Self> {
        // Create data directory if it doesn't exist
        if !Path::new(data_dir).exists() {
            fs::create_dir_all(data_dir)?;
        }

        let db_path = Path::new(data_dir).join("vectors.redb");
        let db = Database::create(&db_path)?;

        // Initialize tables
        {
            let write_txn = db.begin_write()?;
            {
                let _vectors_table = write_txn.open_table(VECTORS_TABLE)?;
                let _metadata_table = write_txn.open_table(METADATA_TABLE)?;
            }
            write_txn.commit()?;
        }

        Ok(Self {
            db: Arc::new(db),
            data_dir: data_dir.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Storage for RedbStorage {
    async fn store_vector(&self, vector: &Vector) -> Result<()> {
        let db = Arc::clone(&self.db);
        let vector = vector.clone();
        
        task::spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            {
                let mut table = write_txn.open_table(VECTORS_TABLE)?;
                let serialized = serde_json::to_vec(&vector)?;
                table.insert(vector.id.as_str(), serialized.as_slice())?;
            }
            write_txn.commit()?;
            Ok::<(), anyhow::Error>(())
        }).await??;

        Ok(())
    }

    async fn get_vector(&self, id: &str) -> Result<Option<Vector>> {
        let db = Arc::clone(&self.db);
        let id = id.to_string();
        
        let result = task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(VECTORS_TABLE)?;
            
            match table.get(id.as_str())? {
                Some(data) => {
                    let vector: Vector = serde_json::from_slice(data.value())?;
                    Ok::<Option<Vector>, anyhow::Error>(Some(vector))
                }
                None => Ok::<Option<Vector>, anyhow::Error>(None),
            }
        }).await??;

        Ok(result)
    }

    async fn delete_vector(&self, id: &str) -> Result<bool> {
        let db = Arc::clone(&self.db);
        let id = id.to_string();
        
        let result = task::spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            let existed = {
                let mut table = write_txn.open_table(VECTORS_TABLE)?;
                let removal_result = table.remove(id.as_str())?;
                removal_result.is_some()
            };
            write_txn.commit()?;
            Ok::<bool, anyhow::Error>(existed)
        }).await??;

        Ok(result)
    }

    async fn count_vectors(&self) -> Result<usize> {
        let db = Arc::clone(&self.db);
        
        let count = task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(VECTORS_TABLE)?;
            Ok::<usize, anyhow::Error>(table.len()? as usize)
        }).await??;

        Ok(count)
    }

    async fn size_bytes(&self) -> Result<usize> {
        let db_path = Path::new(&self.data_dir).join("vectors.redb");
        let metadata = fs::metadata(db_path)?;
        Ok(metadata.len() as usize)
    }

    async fn compact(&self) -> Result<()> {
        // Note: redb Database doesn't need explicit compaction in the same way
        // The database automatically compacts during normal operations
        // For now, we'll just return Ok(()) as a no-op
        // In a real implementation, you might want to trigger a checkpoint or similar operation
        Ok(())
    }

    async fn backup(&self, backup_path: &str) -> Result<()> {
        let source_path = Path::new(&self.data_dir).join("vectors.redb");
        let backup_dir = Path::new(backup_path);
        
        if !backup_dir.exists() {
            fs::create_dir_all(backup_dir)?;
        }
        
        let backup_file = backup_dir.join("vectors.redb");
        fs::copy(source_path, backup_file)?;
        
        Ok(())
    }

    async fn list_collections(&self) -> Result<Vec<String>> {
        let db = self.db.clone();
        
        let collections = task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(VECTORS_TABLE)?;
            
            let mut collections = std::collections::HashSet::new();
            
            for item in table.iter()? {
                let (_, data) = item?;
                let vector: Vector = serde_json::from_slice(data.value())?;
                if let Some(collection) = vector.collection {
                    collections.insert(collection);
                }
            }
            
            Ok::<Vec<String>, anyhow::Error>(collections.into_iter().collect())
        }).await??;

        Ok(collections)
    }

    async fn get_vectors_in_collection(&self, collection: &str) -> Result<Vec<Vector>> {
        let db = self.db.clone();
        let collection = collection.to_string();
        
        let vectors = task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(VECTORS_TABLE)?;
            
            let mut vectors = Vec::new();
            
            for item in table.iter()? {
                let (_, data) = item?;
                let vector: Vector = serde_json::from_slice(data.value())?;
                if vector.collection.as_ref() == Some(&collection) {
                    vectors.push(vector);
                }
            }
            
            Ok::<Vec<Vector>, anyhow::Error>(vectors)
        }).await??;

        Ok(vectors)
    }

    async fn get_first_vector(&self) -> Result<Option<Vector>> {
        let db = Arc::clone(&self.db);
        
        let first_vector = task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(VECTORS_TABLE)?;
            
            let mut iter = table.iter()?;
            let result = if let Some(first) = iter.next() {
                let (_, value) = first?;
                let vector_data = value.value();
                let vector: Vector = serde_json::from_slice(vector_data)?;
                Some(vector)
            } else {
                None
            };
            Ok::<Option<Vector>, anyhow::Error>(result)
        }).await??;

        Ok(first_vector)
    }
}
