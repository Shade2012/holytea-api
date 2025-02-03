-- Add up migration script here
CREATE TABLE payment (
    id SERIAL PRIMARY KEY,
    id_payment_history INT NOT NULL,
    id_product INT NOT NULL,
    payment_product_price INT NOT NULL,
    product_amount SMALLINT NOT NULL
);