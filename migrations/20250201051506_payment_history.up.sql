-- Add up migration script here
CREATE TYPE status_payment_enum AS ENUM ('Selesai', 'Gagal', 'Pending');
CREATE TYPE status_history_enum AS ENUM ('Selesai', 'Gagal', 'Sedang Diproses');
CREATE TABLE payment_history (
    id SERIAL PRIMARY KEY,
    resi_number VARCHAR(255) NOT NULL UNIQUE,
    user_id INT NOT NULL,
    user_amount_money BIGINT NOT NULL,
    total_price BIGINT NOT NULL,
    payment_method VARCHAR(255),
    invoice_url VARCHAR(255),
    invoice_expired TIMESTAMP,
    status_history status_history_enum,
    status_payment status_payment_enum NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP  
);
