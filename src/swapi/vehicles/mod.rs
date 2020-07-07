extern crate reqwest;
use reqwest::Error;
use serde_json::Value;

pub use vehicle::{Vehicle};
pub mod vehicle;

#[derive(Clone)]
pub struct VehicleService {
    url: &'static str
}

impl VehicleService {
    pub fn new(base: &'static str) -> Self {
        Self{
            url: base
        }
    }

    pub fn get_vehicle(&self, id: i32) -> Vehicle {
        #[tokio::main]
        async fn get_vehicle_http(url: String, id: i32) -> Result<Vehicle, Error> {
            let request_url = format!("{}vehicles/{}/", url.to_string(), id);
            let response = reqwest::get(&request_url).await?;
            let p: Vehicle = response.json().await?;

            Ok(p)
        }

        match get_vehicle_http((&self.url).to_string(), id) {
            Ok(vehicle) => vehicle,
            Err(e) => panic!{"{:?}", e}
        }
    }

    pub fn get_vehicles(&self) -> Value {
        #[tokio::main]
        async fn get_people_http(url: String) -> Result<Value, Error> {
            let request_url = format!("{}vehicles/", &url.to_string());
            let response = reqwest::get(&request_url).await?;
            let p: Value = response.json().await?;

            Ok(p)
        }

        match get_people_http((&self.url).to_string()) {
            Ok(vehicles) => vehicles,
            Err(e) => panic!{"{:?}", e}
        }
    }
}