CREATE TABLE merchants (
id SERIAL PRIMARY KEY,
user_id INTEGER NOT NULL,
name VARCHAR NOT NULL,
api_key VARCHAR NOT NULL,
api_secret VARCHAR NOT NULL,
is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
is_active BOOLEAN NOT NULL DEFAULT TRUE,
created_by INTEGER NOT NULL,
created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
updated_by INTEGER DEFAULT NULL,
updated_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_is_active_merchants ON merchants(is_active);
CREATE INDEX idx_user_id_merchants ON merchants(user_id);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';


CREATE TRIGGER update_merchants_modtime
BEFORE UPDATE ON merchants
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();