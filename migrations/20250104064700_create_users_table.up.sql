-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY, -- Auto-incrementing primary key
    username VARCHAR(255) NOT NULL, -- Example: a username field
    email VARCHAR(255) UNIQUE NOT NULL, -- Example: a unique email field
    password TEXT NOT NULL, -- Example: a hashed password field
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);