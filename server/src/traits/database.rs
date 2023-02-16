use actix_web::web;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::{SqliteRow};

pub trait TableName {
    fn table_name() -> String;
}

pub trait ParentTableName {
    fn parent_table_name() -> String;
}

pub trait Query<T> where T:ParentTableName, {
    fn from_sqlite_row(row: SqliteRow, pool: &web::Data<Pool<Sqlite>>) -> T;
}

pub trait Database<T> {
    fn insert(&self, pool: &web::Data<Pool<Sqlite>>) -> i64;

    fn update(&self, pool: &web::Data<Pool<Sqlite>>);

    fn delete(&self, pool: &web::Data<Pool<Sqlite>>);

    fn get_all(pool: &web::Data<Pool<Sqlite>>) -> Vec<T>;

    fn get_by_search(search: Vec<String>, pool: &web::Data<Pool<Sqlite>>) -> Vec<T>;
}
