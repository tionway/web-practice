use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::TutorError;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../server/db_access/mod.rs"]
mod db_access;
#[path = "../server/errors.rs"]
mod errors;
#[path = "../server/handlers/mod.rs"]
mod handlers;
#[path = "../server/models/mod.rs"]
mod models;
#[path = "../server/routes.rs"]
mod routes;
#[path = "../server/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db_pool = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(error) => {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                error.to_string(),
            ))
        }
    };

    let shared_data = web::Data::new(AppState {
        health_check_response: "You've already signed for ".to_string(),
        sign_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                TutorError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");

    HttpServer::new(app).bind(&host_port)?.run().await
}
