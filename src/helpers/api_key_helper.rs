use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct ApiKey {
    pub key: String,
    pub secret: String,
}

pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn generate_api_key() -> ApiKey {
    ApiKey {
        key: generate_random_string(32),
        secret: generate_random_string(64),
    }
}
