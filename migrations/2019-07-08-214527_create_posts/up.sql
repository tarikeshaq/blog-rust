-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE posts (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    message TEXT NOT NULL
);