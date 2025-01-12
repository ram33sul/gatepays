CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    connector_id INTEGER NOT NULL,
    gateway_order_id VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    currency VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    order_secret VARCHAR,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_orders ON orders(is_active);
CREATE INDEX idx_connectory_id_orders ON orders(connector_id);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_orders_modtime
BEFORE UPDATE ON orders
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();