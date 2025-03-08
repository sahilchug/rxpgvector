`cargo pgrx run`

### create extension
```sql
CREATE EXTENSION PGVector;
```

### create table
```sql
CREATE TABLE test_table (
    id SERIAL PRIMARY KEY,
    vec PGVector
);
```

### insert data
```sql
INSERT INTO test_vectors (embedding) VALUES ('{"dims": [1.0, 2.0, 3.0, 4.0]}');
```

### query data
```sql
SELECT * FROM test_vectors;

pgvector=# SELECT * FROM test_vectors;
1 | {"dims":[1.0,2.0,3.0]}
  2 | {"dims":[1.0,2.0,3.0,4.0]}

```

