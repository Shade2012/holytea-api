-- Add up migration script here
CREATE TABLE payment_history {
    id SERIAL PRIMARY KEY,
    user_id INT
}