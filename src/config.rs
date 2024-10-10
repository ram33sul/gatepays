use std::net::IpAddr;

pub struct Config {
    pub database_url: String,
    pub server_host: IpAddr,
    pub server_port: u16,
    pub jwt_secret: String,
    pub paypal_client_id: String,
    pub paypal_secret_key: String,
    pub paypal_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_host: std::env::var("SERVER_HOST")
                .expect("SERVER_HOST must be set")
                .parse()
                .expect("SERVER_HOST must be a valid IP address"),
            server_port: std::env::var("SERVER_PORT")
                .expect("SERVER_PORT must be set")
                .parse()
                .expect("SERVER_PORT must be a valid port number"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .parse()
                .expect("JWT_SECRET must be a valid port number"),
            paypal_client_id: std::env::var("PAYPAL_CLIENT_ID")
                .expect("PAYPAL_CLIENT_ID must be set")
                .parse()
                .expect("PAYPAL_CLIENT_ID must be a valid IP address"),
            paypal_secret_key: std::env::var("PAYPAL_SECRET_KEY")
                .expect("PAYPAL_SECRET_KEY must be set")
                .parse()
                .expect("PAYPAL_SECRET_KEY must be a valid port number"),
            paypal_url: std::env::var("PAYPAL_URL")
                .expect("PAYPAL_URL must be set")
                .parse()
                .expect("PAYPAL_URL must be a valid port number"),
        }
    }
}
