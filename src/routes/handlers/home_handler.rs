use actix_web::{Responder, get, web};

use crate::utils::api_response;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[get("/test")]
pub async fn test() -> impl Responder {
    api_response::ApiResponse::new(200, "Testing".to_string())
}
