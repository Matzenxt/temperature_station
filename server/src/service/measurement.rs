use actix_web::{get, post, HttpResponse, web};
use actix_web::web::{Json};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Sqlite};
use crate::traits::database::Database;
use common::models::measurement::Measurement;

#[post("/measurement")]
pub async fn add_measurement(measurement: Json<Measurement>, pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let id = measurement.insert(&pool);

    HttpResponse::Ok()
        .finish()
}

#[get("/measurement")]
pub async fn get_all_measurements(pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let measurements = Measurement::get_all(&pool);

    HttpResponse::Ok()
        .content_type(ContentType::json().0.essence_str())
        .json(measurements)
}
