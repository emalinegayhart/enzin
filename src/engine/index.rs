use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use serde_json::Value;
use tantivy::{Index, schema::Schema};
use tokio::sync::RwLock;
use crate::error::EnzinError;
use super::schema::infer_schema_from_document;

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

    pub async fn init_schema_from_document(
        &self,
        index_name: &str,
        doc: &Value,
    ) -> Result<(), EnzinError> {
        let index = self.get_index(index_name).await?;
        let current_schema = index.schema();

        if current_schema.fields().count() > 0 {
            return Ok(());
        }

        let new_schema = infer_schema_from_document(doc)
            .map_err(|e| EnzinError::InvalidDocument(e))?;

        let index_path = self.data_dir.join(index_name);
        
        std::fs::remove_dir_all(&index_path)
            .map_err(|e| EnzinError::InternalError(format!("failed to remove old index: {}", e)))?;
        
        std::fs::create_dir_all(&index_path)
            .map_err(|e| EnzinError::InternalError(format!("failed to create index dir: {}", e)))?;

        let new_index = Index::create_in_dir(&index_path, new_schema)
            .map_err(|e| EnzinError::InternalError(format!("failed to create index: {}", e)))?;

        let mut indexes = self.indexes.write().await;
        indexes.insert(index_name.to_string(), new_index);

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

    pub async fn index_documents(
        &self,
        index_name: &str,
        documents: Vec<Value>,
    ) -> Result<usize, EnzinError> {
        if documents.is_empty() {
            return Err(EnzinError::InvalidDocument(
                "no documents provided".to_string(),
            ));
        }

        self.init_schema_from_document(index_name, &documents[0])
            .await?;

        let index = self.get_index(index_name).await?;

        let mut writer = index
            .writer(50_000_000)
            .map_err(|e| EnzinError::InternalError(format!("failed to create writer: {}", e)))?;

        let schema = index.schema();
        let mut indexed_count = 0;

        for doc in documents {
            let mut tantivy_doc = tantivy::doc!();

            match doc.as_object() {
                Some(obj) => {
                    for (key, value) in obj.iter() {
                        if let Ok(field) = schema.get_field(key) {
                            match value {
                                Value::String(s) => {
                                    tantivy_doc.add_text(field, s.clone());
                                }
                                Value::Number(n) => {
                                    if let Some(u) = n.as_u64() {
                                        tantivy_doc.add_u64(field, u);
                                    }
                                }
                                Value::Bool(b) => {
                                    tantivy_doc.add_u64(field, if *b { 1 } else { 0 });
                                }
                                _ => {}
                            }
                        }
                    }
                    writer
                        .add_document(tantivy_doc)
                        .map_err(|e| {
                            EnzinError::InternalError(format!("failed to add document: {}", e))
                        })?;
                    indexed_count += 1;
                }
                None => {
                    return Err(EnzinError::InvalidDocument(
                        "document must be a json object".to_string(),
                    ))
                }
            }
        }

        writer
            .commit()
            .map_err(|e| EnzinError::InternalError(format!("failed to commit: {}", e)))?;

        Ok(indexed_count)
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

    #[tokio::test]
    async fn test_index_documents_single() {
        use serde_json::json;
        
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        
        let docs = vec![json!({
            "title": "hello",
            "count": 42
        })];
        
        let result = manager.index_documents("test", docs).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_index_documents_batch() {
        use serde_json::json;
        
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        
        let docs = vec![
            json!({ "title": "first" }),
            json!({ "title": "second" }),
            json!({ "title": "third" }),
        ];
        
        let result = manager.index_documents("test", docs).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_index_documents_empty() {
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        manager.create_index("test").await.unwrap();
        
        let docs = vec![];
        let result = manager.index_documents("test", docs).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_index_documents_nonexistent_index() {
        use serde_json::json;
        
        let temp_dir = TempDir::new().unwrap();
        let manager = IndexManager::new(temp_dir.path().to_path_buf());

        let docs = vec![json!({ "title": "test" })];
        let result = manager.index_documents("nonexistent", docs).await;
        assert!(result.is_err());
    }
}
