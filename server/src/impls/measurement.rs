use actix_web::web::Data;
use chrono::{DateTime, Utc};
use futures::executor::block_on;
use sqlx::{Pool, Arguments, Sqlite, Row};
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use common::models::measurement::Measurement;
use crate::traits::database::{Database, ParentTableName, Query};

impl ParentTableName for Measurement {
    fn parent_table_name() -> String {
        "measurement".to_string()
    }
}

impl Query<Measurement> for Measurement {
    fn from_sqlite_row(row: SqliteRow, _pool: &Data<Pool<Sqlite>>) -> Measurement {
        Measurement {
            id: row.get(0),
            room: row.get(1),
            device: row.get(2),
            date_time: row.get(3),
            temperature: row.get(4),
            humidity: row.get(5),
        }
    }
}

impl Database<Measurement> for Measurement {
    fn insert(&self, pool: &Data<Pool<Sqlite>>) -> i64 {
        let statement = format!(
            "INSERT INTO {} (room, device, date_time, temperature, humidity) VALUES ($1, $2, $3, $4, $5)",
            Measurement::parent_table_name(),
        );

        let date_time: DateTime<Utc> = Utc::now();

        let mut args = SqliteArguments::default();
        args.add(&self.room);
        args.add(&self.device);
        args.add(date_time);
        args.add(&self.temperature);
        args.add(&self.humidity);

        let mut conn = block_on(pool.acquire()).unwrap();
        let result = block_on(
            sqlx::query_with(
                statement.as_str(),
                args)
                .execute(&mut *conn)
        );

        println!("Insert measurement result:\n {:#?}", result);

        result.unwrap().last_insert_rowid()
    }

    fn update(&self, _pool: &Data<Pool<Sqlite>>) {
        todo!()
    }

    fn delete(&self, _pool: &Data<Pool<Sqlite>>) {
        todo!()
    }

    fn get_all(pool: &Data<Pool<Sqlite>>) -> Vec<Measurement> {
        let mut conn = block_on(pool.acquire()).unwrap();
        let res = block_on(
            sqlx::query(format!("SELECT * FROM {}", Measurement::parent_table_name()).as_str())
                .fetch_all(&mut *conn)
        );
        conn.detach();

        let mut measurements: Vec<Measurement> = Vec::new();

        match res {
            Ok(records) => {
                for record in records {
                    measurements.push(
                        Measurement::from_sqlite_row(record, pool)
                    );
                }
            }
            Err(err) => {
                println!("Error while loading all measuring points:");
                println!("{:#?}", err);
            }
        }

        measurements
    }

    fn get_by_search(search: Vec<String>, pool: &Data<Pool<Sqlite>>) -> Vec<Measurement> {
        let statment = format!("SELECT *
                FROM {} as m
                WHERE m.room = '{}' AND m.date_time <= '{}' AND m.date_time >= '{}'
                ",
                               Measurement::parent_table_name(),
                               search[0],
                               search[1],
                               search[2]
        );

        let mut conn = block_on(pool.acquire()).unwrap();
        let res = block_on(
            sqlx::query(statment.as_str())
            .fetch_all(&mut *conn)
        );
        conn.detach();

        let mut measurements: Vec<Measurement> = Vec::new();

        match res {
            Ok(records) => {
                for record in records {
                    measurements.push(
                        Measurement::from_sqlite_row(record, pool)
                    );
                }
            }
            Err(err) => {
                println!("Error while loading all measuring points for room {} between {} - {}:",
                    search[0], search[1], search[2]
                );
                println!("{:#?}", err);
            }
        }

        measurements
    }
}
