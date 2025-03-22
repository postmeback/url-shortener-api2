mod api;
mod state;
mod model;

use actix_web::{web, App, HttpServer};
use state::AppState;
use std::sync::Mutex;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        url_map: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(api::config) // Register routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
