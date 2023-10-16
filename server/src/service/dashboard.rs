use actix_web::{post, HttpResponse, web};
use actix_web::web::{Json, Path};
use actix_web::http::header::ContentType;
use sqlx::{Pool, Sqlite};
use common::models::dashboard_item::DashboardItem;
use crate::helper::dashboard_item::get_by_room;

#[post("/dashboard/items/{avg}")]
pub async fn get_items_with_avg(rooms: Json<Vec<String>>, avg_duration_seconds: Path<i64>, pool: web::Data<Pool<Sqlite>>) -> HttpResponse {
    let mut dashboard_items: Vec<DashboardItem> = Vec::new();
    let avg = avg_duration_seconds.into_inner();

    for room in rooms.0.iter() {
        match get_by_room(room, avg, &pool) {
            Ok(dashboard_item) => {
                dashboard_items.push(dashboard_item);
            }
            Err(_) => {}
        }
    }

    println!("{:#?}", dashboard_items);

    HttpResponse::Ok()
        .content_type(ContentType::json().0.essence_str())
        .json(dashboard_items)
}
