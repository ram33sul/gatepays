CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER DEFAULT NULL,
    updated_at TIMESTAMP DEFAULT NULL,
);

CREATE INDEX idx_is_active ON users (is_active);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_roles_modtime
BEFORE UPDATE ON roles
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

INSERT INTO roles (id, name, created_by)
VALUES
(1, "system", 1),
(2, "superadmin", 1),
(3, "admin", 1),
(4, "user", 1);