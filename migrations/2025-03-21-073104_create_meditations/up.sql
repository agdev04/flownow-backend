-- Your SQL goes here
CREATE TABLE meditations (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    category_id INTEGER NOT NULL REFERENCES categories(id),
    tags VARCHAR NOT NULL,
    script TEXT NOT NULL,
    image_url VARCHAR NOT NULL,
    audio_url VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)