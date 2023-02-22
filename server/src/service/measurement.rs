use actix_web::{get, post, HttpResponse, web};
use actix_web::web::{Json, Path};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Sqlite};
use crate::traits::database::Database;
use common::models::measurement::Measurement;

#[post("/measurement")]
pub async fn add_measurement(measurement: Json<Measurement>, pool: web::Data<Pool<Sqlite>>) -> HttpResponse {

    println!("Room: {} Device: {} - Temp: {}°C, Humidity: {}%", measurement.room, measurement.device, measurement.temperature, measurement.humidity);

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

#[get("/measurement/{room}/{date_from}/{date_to}")]
pub async fn get_by_search(search_term: Path<(String, String, String)>, pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let inner: (String, String, String) = search_term.into_inner();
    let search: Vec<String> = vec![inner.0, inner.1, inner.2];
    let measurements: Vec<Measurement> = Measurement::get_by_search(search, &pool);

    HttpResponse::Ok()
        .content_type(ContentType::json().0.essence_str())
        .json(measurements)
}
