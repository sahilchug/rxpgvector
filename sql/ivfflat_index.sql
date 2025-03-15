CREATE TABLE ivfflat_index (
    cluster_id serial PRIMARY KEY,
    centroid pgvector,
    vector_ids integer[]
}