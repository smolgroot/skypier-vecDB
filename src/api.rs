use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use skypier_core::{VectorDatabase, Vector};
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

pub type AppState = Arc<VectorDatabase>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRequest {
    pub vectors: Vec<Vector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub vector: Vec<f32>,
    pub k: Option<usize>,
    pub threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_vectors: usize,
    pub dimensions: usize,
    pub storage_size_bytes: usize,
}

pub async fn start_server(db: Arc<VectorDatabase>, port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/stats", get(get_stats))
        .route("/vectors", post(insert_vectors))
        .route("/vectors/:id", get(get_vector))
        .route("/search", post(search_vectors))
        .route("/collections/:collection/search", post(search_in_collection))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting HTTP server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_stats(State(db): State<AppState>) -> Result<Json<StatsResponse>, StatusCode> {
    match db.get_stats().await {
        Ok(stats) => Ok(Json(StatsResponse {
            total_vectors: stats.total_vectors,
            dimensions: stats.dimensions,
            storage_size_bytes: stats.storage_size_bytes,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn insert_vectors(
    State(db): State<AppState>,
    Json(payload): Json<InsertRequest>,
) -> Result<Json<Vec<String>>, StatusCode> {
    match db.insert_vectors(payload.vectors).await {
        Ok(ids) => Ok(Json(ids)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_vector(
    State(db): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vector>, StatusCode> {
    match db.get_vector(&id).await {
        Ok(Some(vector)) => Ok(Json(vector)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn search_vectors(
    State(db): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let k = payload.k.unwrap_or(10);
    let threshold = payload.threshold.unwrap_or(0.0);
    
    match db.search(&payload.vector, k, threshold).await {
        Ok(results) => {
            let search_results = results
                .into_iter()
                .map(|r| SearchResult {
                    id: r.id,
                    score: r.score,
                    metadata: r.metadata,
                })
                .collect();
            Ok(Json(SearchResponse {
                results: search_results,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn search_in_collection(
    State(db): State<AppState>,
    Path(collection): Path<String>,
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let k = payload.k.unwrap_or(10);
    let threshold = payload.threshold.unwrap_or(0.0);
    
    match db.search_in_collection(&collection, &payload.vector, k, threshold).await {
        Ok(results) => {
            let search_results = results
                .into_iter()
                .map(|r| SearchResult {
                    id: r.id,
                    score: r.score,
                    metadata: r.metadata,
                })
                .collect();
            Ok(Json(SearchResponse {
                results: search_results,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        http::StatusCode,
    };
    use axum_test::TestServer;
    use skypier_core::VectorDatabase;
    use std::collections::HashMap;

    async fn create_test_db() -> Arc<VectorDatabase> {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().to_str().unwrap();
        let db = Arc::new(VectorDatabase::new(db_path).await.unwrap());
        // Keep temp_dir alive by not dropping it
        std::mem::forget(temp_dir);
        db
    }

    async fn create_test_app() -> TestServer {
        let db = create_test_db().await;
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/stats", get(get_stats))
            .route("/vectors", post(insert_vectors))
            .route("/vectors/:id", get(get_vector))
            .route("/search", post(search_vectors))
            .route("/collections/:collection/search", post(search_in_collection))
            .layer(CorsLayer::permissive())
            .with_state(db);
        
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn test_health_check() {
        let server = create_test_app().await;
        
        let response = server.get("/health").await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "OK");
    }

    #[tokio::test]
    async fn test_get_stats_empty_db() {
        let server = create_test_app().await;
        
        let response = server.get("/stats").await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let stats: StatsResponse = response.json();
        assert_eq!(stats.total_vectors, 0);
        assert_eq!(stats.dimensions, 0);
    }

    #[tokio::test]
    async fn test_insert_single_vector() {
        let server = create_test_app().await;
        
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let insert_request = InsertRequest {
            vectors: vec![vector],
        };
        
        let response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let ids: Vec<String> = response.json();
        assert_eq!(ids.len(), 1);
        assert!(!ids[0].is_empty());
    }

    #[tokio::test]
    async fn test_insert_multiple_vectors() {
        let server = create_test_app().await;
        
        let vectors = vec![
            Vector::new(vec![1.0, 2.0, 3.0]),
            Vector::new(vec![4.0, 5.0, 6.0]),
            Vector::new(vec![7.0, 8.0, 9.0]),
        ];
        let insert_request = InsertRequest { vectors };
        
        let response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let ids: Vec<String> = response.json();
        assert_eq!(ids.len(), 3);
        assert!(ids.iter().all(|id| !id.is_empty()));
    }

    #[tokio::test]
    async fn test_insert_vector_with_metadata() {
        let server = create_test_app().await;
        
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "test".to_string());
        metadata.insert("category".to_string(), "unit_test".to_string());
        
        let vector = Vector::new(vec![1.0, 2.0, 3.0]).with_metadata(metadata);
        let insert_request = InsertRequest {
            vectors: vec![vector],
        };
        
        let response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let ids: Vec<String> = response.json();
        assert_eq!(ids.len(), 1);
    }

    #[tokio::test]
    async fn test_get_vector_success() {
        let server = create_test_app().await;
        
        // First insert a vector
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let insert_request = InsertRequest {
            vectors: vec![vector.clone()],
        };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        
        let ids: Vec<String> = insert_response.json();
        let vector_id = &ids[0];
        
        // Now get the vector
        let get_response = server
            .get(&format!("/vectors/{}", vector_id))
            .await;
        
        assert_eq!(get_response.status_code(), StatusCode::OK);
        let retrieved_vector: Vector = get_response.json();
        assert_eq!(retrieved_vector.id, *vector_id);
        assert_eq!(retrieved_vector.data, vec![1.0, 2.0, 3.0]);
    }

    #[tokio::test]
    async fn test_get_vector_not_found() {
        let server = create_test_app().await;
        
        let response = server
            .get("/vectors/nonexistent-id")
            .await;
        
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_search_vectors() {
        let server = create_test_app().await;
        
        // Insert test vectors
        let vectors = vec![
            Vector::new(vec![1.0, 0.0, 0.0]),
            Vector::new(vec![0.0, 1.0, 0.0]),
            Vector::new(vec![0.0, 0.0, 1.0]),
        ];
        let insert_request = InsertRequest { vectors };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        assert_eq!(insert_response.status_code(), StatusCode::OK);
        
        // Search for similar vectors
        let search_request = SearchRequest {
            vector: vec![1.0, 0.1, 0.1],
            k: Some(2),
            threshold: Some(0.0),
        };
        
        let search_response = server
            .post("/search")
            .json(&search_request)
            .await;
        
        assert_eq!(search_response.status_code(), StatusCode::OK);
        let search_result: SearchResponse = search_response.json();
        assert!(search_result.results.len() <= 2);
        assert!(!search_result.results.is_empty());
    }

    #[tokio::test]
    async fn test_search_vectors_with_defaults() {
        let server = create_test_app().await;
        
        // Insert test vectors
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let insert_request = InsertRequest {
            vectors: vec![vector],
        };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        assert_eq!(insert_response.status_code(), StatusCode::OK);
        
        // Search with minimal parameters (using defaults)
        let search_request = SearchRequest {
            vector: vec![1.0, 2.0, 3.0],
            k: None, // Should default to 10
            threshold: None, // Should default to 0.0
        };
        
        let search_response = server
            .post("/search")
            .json(&search_request)
            .await;
        
        assert_eq!(search_response.status_code(), StatusCode::OK);
        let search_result: SearchResponse = search_response.json();
        assert_eq!(search_result.results.len(), 1);
    }

    #[tokio::test]
    async fn test_search_in_collection() {
        let server = create_test_app().await;
        
        // Insert vectors in different collections
        let vectors = vec![
            Vector::new(vec![1.0, 0.0, 0.0]).with_collection("collection1".to_string()),
            Vector::new(vec![0.0, 1.0, 0.0]).with_collection("collection2".to_string()),
            Vector::new(vec![0.0, 0.0, 1.0]).with_collection("collection1".to_string()),
        ];
        let insert_request = InsertRequest { vectors };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        assert_eq!(insert_response.status_code(), StatusCode::OK);
        
        // Search in specific collection
        let search_request = SearchRequest {
            vector: vec![1.0, 0.0, 0.0],
            k: Some(10),
            threshold: Some(0.0),
        };
        
        let search_response = server
            .post("/collections/collection1/search")
            .json(&search_request)
            .await;
        
        assert_eq!(search_response.status_code(), StatusCode::OK);
        let search_result: SearchResponse = search_response.json();
        // Should find vectors only from collection1
        assert!(search_result.results.len() <= 2);
    }

    #[tokio::test]
    async fn test_stats_after_insertions() {
        let server = create_test_app().await;
        
        // Insert some vectors
        let vectors = vec![
            Vector::new(vec![1.0, 2.0, 3.0]),
            Vector::new(vec![4.0, 5.0, 6.0]),
        ];
        let insert_request = InsertRequest { vectors };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        assert_eq!(insert_response.status_code(), StatusCode::OK);
        
        // Check stats
        let stats_response = server.get("/stats").await;
        assert_eq!(stats_response.status_code(), StatusCode::OK);
        
        let stats: StatsResponse = stats_response.json();
        assert_eq!(stats.total_vectors, 2);
        assert_eq!(stats.dimensions, 3);
        assert!(stats.storage_size_bytes > 0);
    }

    #[tokio::test]
    async fn test_insert_empty_vectors_list() {
        let server = create_test_app().await;
        
        let insert_request = InsertRequest {
            vectors: vec![],
        };
        
        let response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let ids: Vec<String> = response.json();
        assert_eq!(ids.len(), 0);
    }

    #[tokio::test]
    async fn test_search_empty_database() {
        let server = create_test_app().await;
        
        let search_request = SearchRequest {
            vector: vec![1.0, 2.0, 3.0],
            k: Some(5),
            threshold: Some(0.0),
        };
        
        let response = server
            .post("/search")
            .json(&search_request)
            .await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
        let search_result: SearchResponse = response.json();
        assert_eq!(search_result.results.len(), 0);
    }

    #[tokio::test]
    async fn test_vector_metadata_preserved() {
        let server = create_test_app().await;
        
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "document".to_string());
        metadata.insert("source".to_string(), "test_file.txt".to_string());
        
        let vector = Vector::new(vec![1.0, 2.0, 3.0]).with_metadata(metadata.clone());
        let insert_request = InsertRequest {
            vectors: vec![vector],
        };
        
        let insert_response = server
            .post("/vectors")
            .json(&insert_request)
            .await;
        let ids: Vec<String> = insert_response.json();
        
        // Retrieve the vector and check metadata
        let get_response = server
            .get(&format!("/vectors/{}", ids[0]))
            .await;
        
        let retrieved_vector: Vector = get_response.json();
        assert_eq!(retrieved_vector.metadata, Some(metadata));
    }

    #[tokio::test]
    async fn test_invalid_json_request() {
        let server = create_test_app().await;
        
        let response = server
            .post("/vectors")
            .json(&serde_json::json!({"invalid": "structure"}))
            .await;
        
        // Should return bad request for invalid JSON structure
        assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
