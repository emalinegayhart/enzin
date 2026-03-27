use serde_json::{json, Value};
use tantivy::query::{QueryParser, FuzzyTermQuery, BooleanQuery};
use tantivy::Index;
use tantivy::schema::document::{OwnedValue, TantivyDocument};
use tantivy::Term;
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
    search_with_options(index, query_str, false, 20, 0)
}

pub fn search_with_fuzzy(
    index: &Index,
    query_str: &str,
    fuzzy: bool,
) -> Result<(Vec<SearchResult>, usize), EnzinError> {
    search_with_options(index, query_str, fuzzy, 20, 0)
}

pub fn search_with_options(
    index: &Index,
    query_str: &str,
    fuzzy: bool,
    limit: usize,
    offset: usize,
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

    let query: Box<dyn tantivy::query::Query> = if fuzzy {
        let terms: Vec<_> = query_str.split_whitespace().collect();
        let mut clauses = Vec::new();

        for term_str in terms {
            for field in &text_fields {
                let term = Term::from_field_text(*field, term_str);
                let fuzzy_query = FuzzyTermQuery::new(term, 1, true);
                clauses.push((tantivy::query::Occur::Should, Box::new(fuzzy_query) as Box<dyn tantivy::query::Query>));
            }
        }

        Box::new(BooleanQuery::new(clauses))
    } else {
        let query_parser = QueryParser::for_index(&index, text_fields);
        query_parser.parse_query(query_str).map_err(|e| {
            EnzinError::InvalidDocument(format!("invalid query: {}", e))
        })?
    };

    let max_limit = offset + limit;
    let top_docs = searcher
        .search(&query, &tantivy::collector::TopDocs::with_limit(max_limit))
        .map_err(|e| EnzinError::InternalError(format!("search failed: {}", e)))?;

    let total_count = searcher
        .search(&query, &tantivy::collector::Count)
        .map_err(|e| EnzinError::InternalError(format!("count failed: {}", e)))?;

    let mut results = Vec::new();

    for (i, (score, doc_address)) in top_docs.into_iter().enumerate() {
        if i < offset {
            continue;
        }
        if results.len() >= limit {
            break;
        }
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

    Ok((results, total_count as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tantivy::{schema::Schema, Index};
    use tempfile::TempDir;

    fn create_test_index() -> (TempDir, Index) {
        use tantivy::schema::STORED;
        use tantivy::schema::TEXT;

        let temp_dir = TempDir::new().unwrap();
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT | STORED);

        let schema = schema_builder.build();
        let index = Index::create_in_dir(temp_dir.path(), schema).unwrap();

        (temp_dir, index)
    }

    #[test]
    fn test_basic_search() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();

        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();

        let mut doc1 = tantivy::doc!();
        doc1.add_text(title_field, "hello world");
        doc1.add_text(body_field, "this is a test");
        writer.add_document(doc1).unwrap();

        let mut doc2 = tantivy::doc!();
        doc2.add_text(title_field, "goodbye world");
        doc2.add_text(body_field, "farewell");
        writer.add_document(doc2).unwrap();

        writer.commit().unwrap();

        let (results, total) = search(&index, "hello").unwrap();

        assert_eq!(total, 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_multiple_results() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();
        let body_field = schema.get_field("body").unwrap();

        for i in 0..3 {
            let mut doc = tantivy::doc!();
            doc.add_text(title_field, format!("test document {}", i));
            doc.add_text(body_field, "contains test keyword");
            writer.add_document(doc).unwrap();
        }

        writer.commit().unwrap();

        let (results, total) = search(&index, "test").unwrap();

        assert_eq!(total, 3);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_no_results() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();

        let mut doc = tantivy::doc!();
        doc.add_text(title_field, "hello");
        writer.add_document(doc).unwrap();

        writer.commit().unwrap();

        let (results, total) = search(&index, "xyz").unwrap();

        assert_eq!(total, 0);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_returns_scores() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();

        let mut doc = tantivy::doc!();
        doc.add_text(title_field, "hello hello hello");
        writer.add_document(doc).unwrap();

        writer.commit().unwrap();

        let (results, _) = search(&index, "hello").unwrap();

        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_fuzzy_search_typo() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();

        let mut doc = tantivy::doc!();
        doc.add_text(title_field, "hello");
        writer.add_document(doc).unwrap();

        writer.commit().unwrap();

        let (exact_results, _) = search(&index, "helo").unwrap();
        assert_eq!(exact_results.len(), 0);

        let (fuzzy_results, _) = search_with_fuzzy(&index, "helo", true).unwrap();
        assert_eq!(fuzzy_results.len(), 1);
    }

    #[test]
    fn test_fuzzy_search_multiple_typos() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();

        for word in &["world", "earth", "globe"] {
            let mut doc = tantivy::doc!();
            doc.add_text(title_field, word);
            writer.add_document(doc).unwrap();
        }

        writer.commit().unwrap();

        let (results, total) = search_with_fuzzy(&index, "word", true).unwrap();

        assert!(total > 0);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_fuzzy_search_exact_match_still_works() {
        let (_temp_dir, index) = create_test_index();
        let schema = index.schema();

        let mut writer = index.writer(50_000_000).unwrap();
        let title_field = schema.get_field("title").unwrap();

        let mut doc = tantivy::doc!();
        doc.add_text(title_field, "precise");
        writer.add_document(doc).unwrap();

        writer.commit().unwrap();

        let (exact_results, exact_total) = search(&index, "precise").unwrap();
        let (fuzzy_results, fuzzy_total) = search_with_fuzzy(&index, "precise", true).unwrap();

        assert_eq!(exact_total, 1);
        assert_eq!(fuzzy_total, 1);
        assert_eq!(exact_results.len(), 1);
        assert_eq!(fuzzy_results.len(), 1);
    }

}
