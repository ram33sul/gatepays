use std::net::IpAddr;

pub struct Config {
    pub database_url: String,
    pub server_host: IpAddr,
    pub server_port: u16,
    pub jwt_secret: String,
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
        }
    }
}
