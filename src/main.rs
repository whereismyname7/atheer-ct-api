mod controllers;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use dotenv::dotenv;
use routes::auth::{login_user, register_user};
use sqlx::MySqlPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(register_user)
            .service(login_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
