use std::sync::Mutex;
use std::collections::HashMap;

pub struct AppState {
    pub url_map: Mutex<HashMap<String, String>>, // Stores shortened URLs
}
