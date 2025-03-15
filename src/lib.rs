use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

::pgrx::pg_module_magic!();

/// Define a custom PostgreSQL type called PGVector that stores a vector of f32 values.
/// This type will be used to store the embeddings of the input text.
#[derive(PostgresType, Debug, Serialize, Deserialize)] // This tells PgRX to treat this as a postgres type .
pub struct PGVector {
    dims : Vec<f32>,
}

/// Compute the Euclidean distance (L2) between two vectors.
/// Panics if the vectors have different dimensions.
#[pg_extern]
fn euclidean_distance(v1: PGVector, v2: PGVector) -> f32 {
    if v1.dims.len()!= v2.dims.len(){
        panic!("Vectors must have the same dimension");
    }

    v1.dims
        .iter()
        .zip(v2.dims.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

/// Compute the cosine similarity between two vectors.
/// Returns a value between -1 and 1.
/// Panics if the vectors have different dimensions.
#[pg_extern]
fn vector_cosine_similarity(v1: PGVector, v2: PGVector) -> f32 {
    if v1.dims.len() != v2.dims.len() {
        panic!("Vectors must have the same dimension");
    }
    let dot: f32 = v1.dims.iter().zip(v2.dims.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.dims.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
    let norm2: f32 = v2.dims.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
    dot / (norm1 * norm2)
}


/// A simple function to create a PGVector from a Rust Vec<f32>.
#[pg_extern]
fn create_vector(elements: Vec<f32>) -> PGVector {
    PGVector { dims: elements }
}


/// Prototype function to build an IVFFlat index.
/// `table_name` and `column_name` identify the main table and vector column.
/// `k` is the number of clusters to compute.
#[pg_extern]
fn ivfflat_incex_create(table_name: &str, column_name: &str, k: i32) {
    // 1. Retrieve all vectors from the specified table and column.
    // 2. Run a clustering algorithm (e.g., k-means) on these vectors.
    // 3. For each cluster, compute and store the centroid.
    // 4. Populate the auxiliary table (ivfflat_index) with:
    //    - cluster_id
    //    - centroid
    //    - vector_ids (the IDs from the main table that belong to this cluster)

    info!(
        "Building IVFFlat index on table '{}' column '{}' with k = {}",
        table_name, column_name, k
    );

}

#[pg_extern]
fn ivfflat_index_search(query: PGVector, top_k: i32, probe: i32) -> Vec<i32> {
    // 1. Retrieve the centroids of the k nearest clusters to the input vector.
    // 2. For each centroid, retrieve the vector_ids from the auxiliary table.
    // 3. Return the union of all vector_ids.

    info!(
        "Searching IVFFlat index with query: {:?}, top_k = {}, probe = {}",
        query, top_k, probe
    );

    vec![]
}

// Register the <-> operator.
// This tells PostgreSQL that when it sees the <-> operator on type pgvector,
// it should use our euclidean_distance function.
extension_sql!(
  r#"
CREATE OPERATOR <-> (
    LEFTARG = pgvector,
    RIGHTARG = pgvector,
    PROCEDURE = euclidean_distance,
    COMMUTATOR = <->
 );
    "#,
    name = "pgvector_distance_operator",
    requires = [euclidean_distance]
);
