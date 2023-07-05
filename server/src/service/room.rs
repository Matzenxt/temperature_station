use actix_web::{get, HttpResponse, web};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Sqlite};
use crate::helper;

#[get("/room")]
pub async fn get_all_room(pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let measurements = helper::room::get_all(&pool);

    HttpResponse::Ok()
        .content_type(ContentType::json().0.essence_str())
        .json(measurements)
}
