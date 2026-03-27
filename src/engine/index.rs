use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tantivy::{Index, schema::Schema};
use tokio::sync::RwLock;
use crate::error::EnzinError;

pub struct IndexManager {
    indexes: Arc<RwLock<HashMap<String, Index>>>,
    data_dir: PathBuf,
}

impl IndexManager {
    pub fn new(data_dir: PathBuf) -> Self {
        IndexManager {
            indexes: Arc::new(RwLock::new(HashMap::new())),
            data_dir,
        }
    }

    pub async fn create_index(&self, name: &str) -> Result<(), EnzinError> {
        let mut indexes = self.indexes.write().await;

        if indexes.contains_key(name) {
            return Err(EnzinError::InvalidDocument(format!(
                "index '{}' already exists",
                name
            )));
        }

        let index_path = self.data_dir.join(name);
        std::fs::create_dir_all(&index_path)
            .map_err(|e| EnzinError::InternalError(format!("failed to create index dir: {}", e)))?;

        let schema = Schema::builder().build();
        let index = Index::create_in_dir(&index_path, schema)
            .map_err(|e| EnzinError::InternalError(format!("failed to create index: {}", e)))?;

        indexes.insert(name.to_string(), index);
        Ok(())
    }

    pub async fn list_indexes(&self) -> Vec<String> {
        let indexes = self.indexes.read().await;
        indexes.keys().cloned().collect()
    }

    pub async fn delete_index(&self, name: &str) -> Result<(), EnzinError> {
        let mut indexes = self.indexes.write().await;

        if !indexes.contains_key(name) {
            return Err(EnzinError::IndexNotFound(format!(
                "index '{}' not found",
                name
            )));
        }

        indexes.remove(name);

        let index_path = self.data_dir.join(name);
        std::fs::remove_dir_all(&index_path)
            .map_err(|e| EnzinError::InternalError(format!("failed to delete index dir: {}", e)))?;

        Ok(())
    }

    pub async fn get_index(&self, name: &str) -> Result<Index, EnzinError> {
        let indexes = self.indexes.read().await;

        indexes
            .get(name)
            .cloned()
            .ok_or_else(|| EnzinError::IndexNotFound(format!("index '{}' not found", name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        let result = manager.create_index("test").await;
        assert!(result.is_ok());

        let indexes = manager.list_indexes().await;
        assert_eq!(indexes.len(), 1);
        assert!(indexes.contains(&"test".to_string()));
    }

    #[tokio::test]
    async fn test_create_duplicate_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        let result = manager.create_index("test").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        let result = manager.delete_index("test").await;

        assert!(result.is_ok());
        let indexes = manager.list_indexes().await;
        assert_eq!(indexes.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        let result = manager.delete_index("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        let result = manager.get_index("test").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_nonexistent_index() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        let result = manager.get_index("nonexistent").await;
        assert!(result.is_err());
    }
}
