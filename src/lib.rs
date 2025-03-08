use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

::pgrx::pg_module_magic!();

/// Define a custom PostgreSQL type called PGVector that stores a vector of f32 values.
/// This type will be used to store the embeddings of the input text.
#[derive(PostgresType, Debug, Serialize, Deserialize)] // This tells PgRX to treat this as a postgres type .
pub struct PGVector {
    dims : Vec<f32>,
}

/// A simple function to create a PGVector from a Rust Vec<f32>.
#[pg_extern]
fn create_vector(elements: Vec<f32>) -> PGVector {
    PGVector { dims: elements }
}



/*
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pgvector() {
        assert_eq!("Hello, pgvector", crate::hello_pgvector());
    }

}



/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

 */
