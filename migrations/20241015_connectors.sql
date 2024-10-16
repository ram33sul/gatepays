CREATE TABLE connectors (
    id SERIAL PRIMARY KEY,
    merchant_id INTEGER NOT NULL,
    gateway_id INTEGER NOT NULL,
    gateway_api_key VARCHAR NOT NULL,
    gateway_api_secret VARCHAR NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_connectors ON connectors (is_active);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_connectors_modtime
BEFORE UPDATE ON connectors
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();