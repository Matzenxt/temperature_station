use std::ops::{Add, Sub};
use crate::traits::database::ParentTableName;
use actix_web::web::Data;
use chrono::{DateTime, Duration, Utc};
use common::models::dashboard_item::DashboardItem;
use common::models::measurement::Measurement;
use futures::executor::block_on;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Row, Sqlite};

fn from_sqlite_row(row: SqliteRow) -> DashboardItem {
    let room: String = row.get(0);

    return DashboardItem {
        room_id: 0,
        room_name: "".to_string(),
        last_time: Default::default(),
        temperature: 0.0,
        humidity: 0.0,
        avg_duration_seconds: 0,
        avg_temperature: 0.0,
        avg_humidity: 0.0,
    };
}

// TODO:
pub fn get_by_room(room: String, avg_duration_seconds: i64, pool: &Data<Pool<Sqlite>>) -> Result<DashboardItem, Error> {
    let end_time: DateTime<Utc> = Utc::now();
    let start_time: DateTime<Utc> = end_time.sub(Duration::seconds(avg_duration_seconds));

    let mut conn = block_on(pool.acquire()).unwrap();
    let res = block_on(
        sqlx::query(
            format!(
                "SELECT m.id, m.room, AVG(m.temperature), AVG(m.humidity)
                FROM {} as m
                WHERE m.room = '{}' AND m.date_time <= '{}' AND m.date_time >= '{}'
                ",
                Measurement::parent_table_name(),
                room,
                start_time,
                end_time,
            )
            .as_str(),
        )
        .fetch_one(&mut *conn),
    );
    conn.detach();

    match res {
        Ok(record) => {
            Ok(DashboardItem {
                room_id: 0,
                room_name: "".to_string(),
                last_time: Default::default(),
                temperature: 0.0,
                humidity: 0.0,
                avg_duration_seconds,
                avg_temperature: 0.0,
                avg_humidity: 0.0,
            })
        }
        Err(err) => {
            println!("Error while loading all rooms:");
            println!("{:#?}", err);
            Err(Error::Decode(Box::new((err))))
        }
    }
}
