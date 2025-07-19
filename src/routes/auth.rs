use crate::controllers::auth_controller::{login, register};
use crate::models::user::User;
use actix_web::{Responder, post, web};
use sqlx::MySqlPool;

#[post("/api/register")]
pub async fn register_user(user: web::Json<User>, db: web::Data<MySqlPool>) -> impl Responder {
    register(user, db).await
}

#[post("/api/login")]
pub async fn login_user(user: web::Json<User>, db: web::Data<MySqlPool>) -> impl Responder {
    login(user, db).await
}
