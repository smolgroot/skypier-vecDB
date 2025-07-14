use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skypier_core::VectorDatabase;
use skypier_storage::Vector;
use std::sync::Arc;
use tokio::runtime::Runtime;

async fn setup_db() -> Arc<VectorDatabase> {
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().to_str().unwrap();
    let db = Arc::new(VectorDatabase::new(db_path).await.unwrap());
    std::mem::forget(temp_dir); // Keep temp dir alive
    db
}

fn insert_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(setup_db());

    c.bench_function("insert_single_vector", |b| {
        b.iter(|| {
            rt.block_on(async {
                let vector = Vector::new(black_box(vec![1.0, 2.0, 3.0, 4.0]));
                let _ = db.insert_vectors(vec![vector]).await;
            })
        })
    });
}

fn search_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(setup_db());

    // Prepare data
    rt.block_on(async {
        let vectors: Vec<Vector> = (0..1000)
            .map(|i| Vector::new(vec![i as f32, (i * 2) as f32, (i * 3) as f32]))
            .collect();
        let _ = db.insert_vectors(vectors).await;
    });

    c.bench_function("search_1000_vectors", |b| {
        b.iter(|| {
            rt.block_on(async {
                let query = black_box(vec![500.0, 1000.0, 1500.0]);
                let _ = db.search(&query, 10, 0.0).await;
            })
        })
    });
}

fn batch_insert_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("insert_batch_100_vectors", |b| {
        b.iter(|| {
            rt.block_on(async {
                let db = setup_db().await;
                let vectors: Vec<Vector> = (0..100)
                    .map(|i| Vector::new(black_box(vec![i as f32, (i * 2) as f32, (i * 3) as f32])))
                    .collect();
                let _ = db.insert_vectors(vectors).await;
            })
        })
    });
}

criterion_group!(
    benches,
    insert_benchmark,
    search_benchmark,
    batch_insert_benchmark
);
criterion_main!(benches);
