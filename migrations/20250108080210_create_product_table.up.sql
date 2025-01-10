-- Add up migration script here
CREATE TABLE products (
    id SERIAL PRIMARY KEY, -- Auto-incrementing primary key
    product_name VARCHAR(255) NOT NULL, -- Example: a nama field
    product_image TEXT, -- Example: a image field
    product_stock SMALLINT NOT NULL, -- Example: a stock field
    product_available BOOLEAN DEFAULT TRUE NOT NULL, -- Example: a avaible field
    product_price INTEGER NOT NULL, -- Example: a price field
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);