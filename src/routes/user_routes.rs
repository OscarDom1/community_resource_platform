use actix_web::{get, post, put, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{prelude::FromRow, PgPool};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime> // Add this if created_at is a nullable field
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register_user(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        req.name,
        req.email,
        hashed_password,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(e) => {
            eprintln!("Error inserting user: {:?}", e);  // Log the error details
            HttpResponse::InternalServerError().body(format!("Error: {}", e))  // Return error details in the response body
        }
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
        "SELECT id, name, email, password, created_at FROM users WHERE email = $1",
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

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[put("/update/{id}")]
pub async fn update_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let hashed_password = match &req.password {
        Some(password) => Some(hash(&password, DEFAULT_COST).expect(&password)),
        None => None,
    };

    let result = sqlx::query!(
        "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email), password = COALESCE($3, password) WHERE id = $4",
        req.name,
        req.email,
        hashed_password,
        user_id,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User updated successfully"),
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}


#[get("/")]
async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().json("Fetch all users")
}

#[post("/")]
async fn add_user() -> impl Responder {
    HttpResponse::Created().finish()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // Registering the routes for get_all_users, add_user, login, and register
    cfg.service(get_all_users)
        .service(add_user)
        .service(register_user)
        .service(login)
        .service(update_user);
}