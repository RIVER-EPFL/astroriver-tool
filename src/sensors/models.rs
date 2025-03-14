use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sensor {
    pub serial_number: String,
    pub model: String,
    pub parameter_id: Uuid,
    pub calibrations: Vec<Calibration>,
    pub id: Uuid,
    pub parameter: Parameter,
    pub field_id: String,
    pub station_link: Vec<StationLink>,
    pub history: Option<()>,
    pub current_assignment: Option<StationLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,
    pub acronym: String,
    pub unit: String,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calibration {
    pub calibrated_on: DateTime<Utc>,
    pub slope: f64,
    pub intercept: f64,
    pub min_range: f64,
    pub max_range: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StationLink {
    pub station_id: Uuid,
    pub installed_on: NaiveDateTime,
    pub sensor_id: Uuid,
    pub id: Uuid,
    pub sensor_position: i32,
    pub iterator: i32,
}
