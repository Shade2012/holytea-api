-- Add down migration script here
DROP TABLE payment_history;
DROP TYPE status_payment_enum;
DROP TYPE status_history_enum;