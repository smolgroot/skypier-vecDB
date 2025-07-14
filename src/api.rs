use axum::{
    extract::{Path, Query, State},
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
use uuid::Uuid;

pub type AppState = Arc<VectorDatabase>;

#[derive(Debug, Deserialize)]
pub struct InsertRequest {
    pub vectors: Vec<Vector>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub vector: Vec<f32>,
    pub k: Option<usize>,
    pub threshold: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
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
