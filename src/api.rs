use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::state::AppState;
use crate::model::{UrlRequest, UrlResponse};

#[post("/api/shorten")]
async fn shorten_url(data: web::Data<AppState>, req: web::Json<UrlRequest>) -> impl Responder {
    let mut url_map = data.url_map.lock().unwrap();
    let short_code = Uuid::new_v4().to_string().chars().take(6).collect::<String>();
    url_map.insert(short_code.clone(), req.long_url.clone());

    HttpResponse::Ok().json(UrlResponse {
        short_url: format!("http://localhost:8080/{}", short_code),
    })
}

// Function to register routes
pub fn config(cfg: &mut web::ServiceConfig) {
cfg.service(shorten_url);
}
