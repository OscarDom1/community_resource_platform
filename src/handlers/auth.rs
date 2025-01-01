use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let hashed_password = hash(&req.password, DEFAULT_COST).expect("Failed to hash password");

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        req.name,
        req.email,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let user = sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users WHERE email = $1",
        req.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => {
            if verify(&req.password, &user.password).unwrap_or(false) {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
