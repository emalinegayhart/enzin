use serde_json::{json, Value};
use tantivy::query::QueryParser;
use tantivy::Index;
use tantivy::schema::document::{OwnedValue, TantivyDocument};
use crate::error::EnzinError;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: Value,
    pub score: f32,
}

pub struct SearchResponse {
    pub query: String,
    pub hits: Vec<SearchResult>,
    pub total: usize,
    pub took_ms: u128,
}

pub fn search(
    index: &Index,
    query_str: &str,
) -> Result<(Vec<SearchResult>, usize), EnzinError> {
    let schema = index.schema();
    let searcher = index
        .reader()
        .map_err(|e| EnzinError::InternalError(format!("failed to create reader: {}", e)))?
        .searcher();

    let text_fields: Vec<_> = schema
        .fields()
        .filter_map(|(field, entry)| {
            if entry.is_indexed() {
                Some(field)
            } else {
                None
            }
        })
        .collect();

    if text_fields.is_empty() {
        return Ok((vec![], 0));
    }

    let query_parser = QueryParser::for_index(&index, text_fields);
    let query = query_parser.parse_query(query_str).map_err(|e| {
        EnzinError::InvalidDocument(format!("invalid query: {}", e))
    })?;

    let top_docs = searcher
        .search(&query, &tantivy::collector::TopDocs::with_limit(1000))
        .map_err(|e| EnzinError::InternalError(format!("search failed: {}", e)))?;

    let total = top_docs.len();
    let mut results = Vec::new();

    for (score, doc_address) in top_docs {
        let tantivy_doc: TantivyDocument = searcher
            .doc(doc_address)
            .map_err(|e| EnzinError::InternalError(format!("failed to retrieve doc: {}", e)))?;

        let mut doc_json = json!({});

        for field_value in tantivy_doc {
            let field = field_value.field();
            let field_entry = schema.get_field_entry(field);
            let field_name = field_entry.name();
            let value = field_value.value();

            let json_value = match value {
                OwnedValue::Str(s) => Value::String(s.clone()),
                OwnedValue::U64(u) => Value::Number((*u).into()),
                OwnedValue::I64(i) => Value::Number((*i).into()),
                _ => Value::Null,
            };
            doc_json[field_name] = json_value;
        }

        results.push(SearchResult {
            document: doc_json,
            score,
        });
    }

    Ok((results, total))
}
