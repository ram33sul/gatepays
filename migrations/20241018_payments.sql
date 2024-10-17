CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL,
    status VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    amount_received INTEGER NOT NULL,
    net_amount INTEGER NOT NULL,
    currency VARCHAR(3) NOT NULL,
    payment_method VARCHAR NOT NULL,
    customer_id INTEGER NOT NULL,
    description VARCHAR DEFAULT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_payments ON payments(is_active);
CREATE INDEX idx_order_id_payments ON payments(order_id);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_payments_modtime
BEFORE UPDATE ON payments
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();