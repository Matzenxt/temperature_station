use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DashboardItem {
    pub room_id: i64,
    pub room_name: String,
    pub last_time: DateTime<chrono::Utc>,
    pub temperature: f64,
    pub humidity: f64,
    pub avg_duration_seconds: i64,
    pub avg_temperature: f64,
    pub avg_humidity: f64,
}
