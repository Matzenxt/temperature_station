use std::ops::Sub;
use crate::traits::database::ParentTableName;
use actix_web::web::Data;
use chrono::{DateTime, Duration, Utc};
use common::models::dashboard_item::DashboardItem;
use common::models::measurement::Measurement;
use futures::executor::block_on;
use sqlx::{Error, Pool, Row, Sqlite};

pub fn get_by_room(room: &String, avg_duration_seconds: i64, pool: &Data<Pool<Sqlite>>) -> Result<DashboardItem, Error> {
    let end_time: DateTime<Utc> = Utc::now();
    let start_time: DateTime<Utc> = end_time.sub(Duration::seconds(avg_duration_seconds*100));

    let mut conn = block_on(pool.acquire()).unwrap();
    let avg_res = block_on(
        sqlx::query(
            format!(
                "SELECT m.id, m.room, AVG(m.temperature), AVG(m.humidity)
                FROM {} as m
                WHERE m.room = '{}' AND m.date_time >= '{}' AND m.date_time <= '{}'
                ",
                Measurement::parent_table_name(),
                room,
                start_time.to_rfc3339(),
                end_time.to_rfc3339(),
            )
            .as_str(),
        )
        .fetch_one(&mut *conn),
    );

    let last_res = block_on(
        sqlx::query(
            format!(
                "SELECT m.room, m.date_time, m.temperature, m.humidity
                FROM {} as m
                WHERE m.room = '{}'
                ORDER BY m.date_time DESC
                ",
                Measurement::parent_table_name(),
                room,
            )
                .as_str(),
        )
            .fetch_one(&mut *conn),
    );

    conn.detach();

    if avg_res.is_ok() && last_res.is_ok() {
        let avg_record = avg_res.unwrap();
        let last_record = last_res.unwrap();

        Ok(DashboardItem {
            room_id: 0,
            room_name: last_record.get(0),
            last_time: last_record.get(1),
            temperature: last_record.get(2),
            humidity: last_record.get(3),
            avg_duration_seconds,
            avg_temperature: avg_record.get(2),
            avg_humidity: avg_record.get(3),
        })
    } else {
        println!("Error while loading dashboard item:");

        Err(Error::Decode(Box::new((avg_res.err().unwrap()))))
    }

}
