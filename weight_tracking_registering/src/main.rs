use std::io::{self, BufRead};
use std::time::Duration;
use color_eyre::Result;
use serde::{Serialize, Deserialize};
use ProcessingError::*;

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
struct NewMeasurement {
    weight: f64,
}

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
struct Measurement {
    id: i32,
    weight: f64,
    measured_at: chrono::NaiveDateTime,
}

const BASE_URL: &str = env!("BASE_URL");

#[tokio::main]
async fn main() -> Result<()> {
    println!("How much did you weigh yourself to be? (in kg): ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        // println!("{:#?}", line);
        match process(line).await {
            Ok(measurement) => {
                println!();
                println!("The measurement has been successfully registered as {}kg.", measurement.weight);
                tokio::time::sleep(Duration::from_secs(2)).await;
                return Ok(());
            }
            Err(ParseError) => {
                println!("Invalid number entered");
            }
            Err(RequestError) => {
                println!("Could not send request");
            }
            Err(ResponseError) => {
                println!("Unknown server error");
            }
        }
        println!();
        println!("How much did you weigh yourself to be? (in kg): ");
    }
    panic!("Unreachable code reached :(");
}

#[derive(Debug)]
enum ProcessingError {
    ParseError,
    RequestError,
    ResponseError,
}

async fn process(line: String) -> Result<Measurement, ProcessingError> {
    let client = reqwest::Client::new();

    // println!("{:#?}", line);
    let Ok(weight) = line.parse::<f64>() else {
        return Err(ParseError)
    };

    // println!("{:#?}", weight);
    let measurement = NewMeasurement {
        weight
    };

    // println!("{:#?}", measurement);
    let url = format!("{BASE_URL}/measurements");
    let request_result = client
        .post(url)
        .json(&measurement)
        .send()
        .await;

    // println!("{:#?}", request_result);
    let Ok(response) = request_result else {
        return Err(RequestError)
    };
    let Ok(response) = response.error_for_status() else {
        return Err(ResponseError)
    };

    response
        .json::<Measurement>()
        .await
        .map_err(|_| ResponseError)
}
