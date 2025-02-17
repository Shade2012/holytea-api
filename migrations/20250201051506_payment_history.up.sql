-- Add up migration script here
CREATE TYPE status_payment_enum AS ENUM ('Selesai', 'Gagal', 'Pending');
CREATE TABLE payment_history (
    id SERIAL PRIMARY KEY,
    resi_number VARCHAR(255) NOT NULL UNIQUE,
    user_id INT NOT NULL,
    user_amount_money BIGINT NOT NULL,
    total_price BIGINT NOT NULL,
    status_payment status_payment_enum NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
