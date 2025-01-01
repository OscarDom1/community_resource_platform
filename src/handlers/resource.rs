use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: String,
    pub available: Option<bool>,

}

#[post("/resources")]
pub async fn create_resource(
    pool: web::Data<PgPool>,
    req: web::Json<CreateResourceRequest>,
    user_id: web::ReqData<i32>, // Assume middleware sets this
) -> impl Responder {
    let available = req.available.unwrap_or(true);
    let result = sqlx::query!(
        "INSERT INTO resources (title, description, owner_id) VALUES ($1, $2, $3)",
        req.title,
        req.description,
        *user_id,
        available
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/resources")]
pub async fn list_resources(pool: web::Data<PgPool>) -> impl Responder {
    let resources = sqlx::query_as!(
        crate::models::resource::Resource,
        "SELECT * FROM resources"
    )
    .fetch_all(pool.get_ref())
    .await;

    match resources {
        Ok(resources) => HttpResponse::Ok().json(resources),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
