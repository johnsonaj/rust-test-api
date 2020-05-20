extern crate reqwest;
use reqwest::Error;
use serde_json::Value;

pub use vehicle::{Vehicle};
pub mod vehicle;

#[derive(Clone)]
pub struct VehicleService {
    url: String
}

impl VehicleService {
    pub fn new(base: String) -> VehicleService {
        let mut new_base: String = base.to_owned();
        let route: &str = "vehicles";
        new_base.push_str(route);
        VehicleService{url: new_base}
    }

    pub fn get_vehicle(&self, id: i32) -> Vehicle {
        #[tokio::main]
        async fn get_vehicle_http(url: String, id: i32) -> Result<Vehicle, Error> {
            let request_url = format!("{}/{}", url.to_string(), id);
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
            let response = reqwest::get(&url.to_string()).await?;
            let p: Value = response.json().await?;

            Ok(p)
        }

        match get_people_http((&self.url).to_string()) {
            Ok(vehicles) => vehicles,
            Err(e) => panic!{"{:?}", e}
        }
    }
}