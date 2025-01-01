use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid; // Import Uuid

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Resource {
    pub id: Uuid, // Assuming ID is of type i32
    pub title: String,
    pub description: String,
    pub available: bool,
    pub owner_id: i32, 
    pub created_at: Option<NaiveDateTime>, 
}

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: String,
    pub available: Option<bool>,
    pub created_at: Option<NaiveDateTime>, // Include created_at in the request
}

#[post("/resources")]
pub async fn create_resource(
    pool: web::Data<PgPool>,
    req: web::Json<CreateResourceRequest>,
    user_id: web::ReqData<i32>, // Assume middleware sets this
) -> impl Responder {
    let available = req.available.unwrap_or(true); // Default to true if available is None

    // If created_at is not provided, it will be None
    let created_at = req.created_at.clone(); 

    // Insert the resource into the database
    let result = sqlx::query!(
        "INSERT INTO resources (title, description, available, owner_id, created_at) VALUES ($1, $2, $3, $4, $5)",
        req.title,
        req.description,
        available,
        *user_id,
        created_at // Use the provided created_at or None if it's not provided
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            log::error!("Error inserting resource: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/resources")]
pub async fn list_resources(pool: web::Data<PgPool>) -> impl Responder {
    let resources = sqlx::query_as!(
        crate::models::resource::Resource,
        "SELECT id, title, description, available, owner_id, created_at FROM resources"
    )
    .fetch_all(pool.get_ref())
    .await;

    match resources {
        Ok(resources) => HttpResponse::Ok().json(resources),
        Err(e) => {
            log::error!("Error fetching resources: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_resource)
       .service(list_resources);
}
