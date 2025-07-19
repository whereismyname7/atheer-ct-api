use crate::models::{response::ApiResponse, user::User};
use actix_web::{HttpResponse, web};
use bcrypt::verify;
use sqlx::MySqlPool;
use validator::Validate;

pub async fn register(user: web::Json<User>, db: web::Data<MySqlPool>) -> HttpResponse {
    // Validate email and password first
    if let Err(errors) = user.validate() {
        if errors.field_errors().contains_key("email") {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                message: "Invalid email address".into(),
                code: 400,
                data: None,
            });
        }
        if errors.field_errors().contains_key("password") {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                message: "Invalid password. Password must be at least 8 characters and include upper/lower case, number, and special character".into(),
                code: 400,
                data: None,
            });
        }
    }

    // Check if email already exists
    let existing = sqlx::query!("SELECT id FROM users WHERE email = ?", user.email)
        .fetch_optional(db.get_ref())
        .await;

    if let Ok(Some(_)) = existing {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            message: "Username already exists".into(),
            code: 400,
            data: None,
        });
    }

    // Hash password
    let hashed_password = match bcrypt::hash(&user.password, bcrypt::DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: "Password hashing failed".into(),
                code: 500,
                data: None,
            });
        }
    };

    // Insert new user
    let result = sqlx::query!(
        "INSERT INTO users (email, password) VALUES (?, ?)",
        user.email,
        hashed_password
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(ApiResponse::<()> {
            success: true,
            message: "Account created successfully".into(),
            code: 201,
            data: None,
        }),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: "Error creating user".into(),
                code: 500,
                data: None,
            })
        }
    }
}

pub async fn login(user: web::Json<User>, db: web::Data<MySqlPool>) -> HttpResponse {
    let result = sqlx::query!("SELECT password FROM users WHERE email = ?", user.email)
        .fetch_optional(db.get_ref())
        .await;

    match result {
        Ok(Some(record)) => {
            if verify(&user.password, &record.password).unwrap_or(false) {
                HttpResponse::Ok().json(ApiResponse::<()> {
                    success: true,
                    code: 200,
                    message: "Logged in successfully".into(),
                    data: None,
                })
            } else {
                HttpResponse::Unauthorized().json(ApiResponse::<()> {
                    success: false,
                    code: 401,
                    message: "Invalid username or password".into(),
                    data: None,
                })
            }
        }
        _ => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            success: false,
            code: 401,
            message: "Invalid username or password".into(),
            data: None,
        }),
    }
}
