use chrono::DateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Measurement {
    pub id: i64,
    pub room: String,
    pub device: String,
    pub date_time: DateTime<chrono::Utc>,
    pub temperature: f32,
    pub humidity: f32,
}
