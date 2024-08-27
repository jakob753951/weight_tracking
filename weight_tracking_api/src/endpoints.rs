use poem::{Result, web::Data};
use poem::error::InternalServerError;
use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use sqlx::PgPool;

use crate::models::*;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all Measurements
    #[oai(path = "/measurements", method = "get")]
    async fn get_measurements(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Measurement>>> {
        let recs = sqlx::query_as(
            r#"
SELECT id, weight, measured_at
FROM measurements
ORDER BY measured_at
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(recs))
    }

    /// Insert Measurement
    #[oai(path = "/measurements", method = "post")]
    pub async fn create_measurement(&self, pool: Data<&PgPool>, measurement: Json<NewMeasurement>) -> Result<Json<Measurement>> {
        let measurement = sqlx::query_as(
            r"
                INSERT INTO measurements (weight)
                VALUES ($1)
                RETURNING id, weight, measured_at
            ")
            .bind(&measurement.weight)
            .fetch_one(pool.0)
            .await
            .map_err(InternalServerError)?;

        println!("Measurement created: {measurement:#?}");

        Ok(Json(measurement))
    }
}