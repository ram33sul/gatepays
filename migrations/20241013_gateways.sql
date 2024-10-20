CREATE TABLE gateways (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL CHECK (name IN ('PAYPAL', 'RAZORPAY', 'STRIPE')),
    url VARCHAR NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_gateways ON gateways (is_active);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_gateways_modtime
BEFORE UPDATE ON gateways
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

INSERT INTO gateways (id, name, url, created_by)
VALUES
(1, 'PAYPAL', 'https://api-m.sandbox.paypal.com', 1),
(2, 'RAZORPAY', '', 1),
(3, 'STRIPE', 'https://api.stripe.com/v1', 1);