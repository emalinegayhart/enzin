use serde_json::Value;
use tantivy::schema::{Schema, STORED, TEXT};

pub fn infer_schema_from_document(doc: &Value) -> Result<Schema, String> {
    let mut schema_builder = Schema::builder();

    match doc.as_object() {
        Some(obj) => {
            for (key, value) in obj.iter() {
                match value {
                    Value::String(_) => {
                        schema_builder.add_text_field(key, TEXT | STORED);
                    }
                    Value::Number(_) => {
                        schema_builder.add_u64_field(key, STORED);
                    }
                    Value::Bool(_) => {
                        schema_builder.add_u64_field(key, STORED);
                    }
                    _ => {
                        return Err(format!("unsupported field type for '{}'", key));
                    }
                }
            }
            Ok(schema_builder.build())
        }
        None => Err("document must be a json object".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_infer_string_field() {
        let doc = json!({ "title": "test" });
        let schema = infer_schema_from_document(&doc).unwrap();
        assert_eq!(schema.fields().count(), 1);
    }

    #[test]
    fn test_infer_number_field() {
        let doc = json!({ "count": 42 });
        let schema = infer_schema_from_document(&doc).unwrap();
        assert_eq!(schema.fields().count(), 1);
    }

    #[test]
    fn test_infer_bool_field() {
        let doc = json!({ "active": true });
        let schema = infer_schema_from_document(&doc).unwrap();
        assert_eq!(schema.fields().count(), 1);
    }

    #[test]
    fn test_infer_mixed_schema() {
        let doc = json!({
            "title": "test",
            "count": 42,
            "active": true
        });
        let schema = infer_schema_from_document(&doc).unwrap();
        assert_eq!(schema.fields().count(), 3);
    }

    #[test]
    fn test_infer_unsupported_type() {
        let doc = json!({ "data": [1, 2, 3] });
        let result = infer_schema_from_document(&doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_infer_non_object() {
        let doc = json!([1, 2, 3]);
        let result = infer_schema_from_document(&doc);
        assert!(result.is_err());
    }
}
