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
