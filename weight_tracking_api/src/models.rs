use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;

#[derive(Object)]
#[derive(FromRow)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Measurement {
    pub(crate) id: i32,
    pub(crate) weight: f64,
    pub(crate) measured_at: chrono::NaiveDateTime,
}

#[derive(Object)]
#[derive(FromRow)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct NewMeasurement {
    pub(crate) weight: f64,
}