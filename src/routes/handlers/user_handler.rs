use actix_web::{Responder, web};

use crate::utils::{api_response, app_state};

#[get("")]
pub async fn user(app_state: web::Data<app_state::AppState>) -> impl Responder {
    // Implement user logic here
    todo!();

    api_response::ApiResponse::new(200, "".to_string())
}
