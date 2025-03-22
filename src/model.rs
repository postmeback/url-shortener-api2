use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UrlRequest {
    pub long_url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_url: String,
}
