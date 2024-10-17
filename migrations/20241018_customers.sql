CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    merchant_id INTEGER NOT NULL,
    name VARCHAR,
    email VARCHAR,
    phone_country_code VARCHAR,
    phone VARCHAR,
    address_id INTEGER,
    description VARCHAR,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_customers ON customers(is_active);
CREATE INDEX idx_email_customers ON customers(email);
CREATE INDEX idx_address_id_customers ON customers(address_id);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_customers_modtime
BEFORE UPDATE ON customers
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();