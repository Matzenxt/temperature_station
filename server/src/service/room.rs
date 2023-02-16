use actix_web::{get, post, HttpResponse, web};
use actix_web::web::{Json, Path};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Sqlite};
use crate::traits::database::Database;
use common::models::measurement::Measurement;
use crate::helper;

#[get("/room")]
pub async fn get_all_room(pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let measurements = helper::room::get_all(&pool);

    HttpResponse::Ok()
        .content_type(ContentType::json().0.essence_str())
        .json(measurements)
}
