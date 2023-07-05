use actix_web::web::Data;
use futures::executor::block_on;
use sqlx::{Pool, Row, Sqlite};
use sqlx::sqlite::SqliteRow;
use common::models::measurement::Measurement;
use crate::traits::database::ParentTableName;

fn from_sqlite_row(row: SqliteRow) -> String {
    let room: String = row.get(0);
    return room
}

pub fn get_all(pool: &Data<Pool<Sqlite>>) -> Vec<String> {
    let mut conn = block_on(pool.acquire()).unwrap();
    let res = block_on(
        sqlx::query(format!("
                SELECT m.room
                FROM {} AS m
                GROUP BY m.room",
                            Measurement::parent_table_name()).as_str())
            .fetch_all(&mut *conn)
    );
    conn.detach();

    let mut room: Vec<String> = Vec::new();

    match res {
        Ok(records) => {
            for record in records {
                room.push(
                    from_sqlite_row(record)
                );
            }
        }
        Err(err) => {
            println!("Error while loading all rooms:");
            println!("{:#?}", err);
        }
    }

    room
}
