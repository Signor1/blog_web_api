use actix_web::{Responder, get, web};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response, app_state::AppState};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {
    let res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user;",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, "Testing".to_string())
}
