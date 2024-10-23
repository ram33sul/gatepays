use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: u64,
    pub page_size: u64,
}
