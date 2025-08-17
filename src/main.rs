use actix_web::{App, HttpServer, middleware::Logger, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env::{set_var, var_os};

use crate::utils::app_state::AppState;

mod routes;
mod utils;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    if var_os("RUST_LOG").is_none() {
        unsafe {
            set_var("RUST_LOG", "actix_web=info");
        };
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
