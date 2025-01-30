CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    sample_type VARCHAR NOT NULL,
    collected_at TIMESTAMP NOT NULL,
    status VARCHAR NOT NULL
);
