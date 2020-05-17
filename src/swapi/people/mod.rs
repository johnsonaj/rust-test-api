extern crate reqwest;
use reqwest::Error;
use serde_json::Value;

mod person;
use person::{Person};
use super::vehicles::VehicleService;

#[derive(Clone)]
pub struct PeopleService {
    url: String,
    vehicle_svc: VehicleService
}

impl PeopleService {
    pub fn new(base: String, vehicle_svc: VehicleService) -> PeopleService {
        let mut new_base: String = base.to_owned();
        let route: &str = "people";
        new_base.push_str(route);
        PeopleService{
            url: new_base,
            vehicle_svc: vehicle_svc
        }
    }

    pub fn get_person(&self, id: i32) -> Person {
        #[tokio::main]
        async fn get_person_http(url: String, id: i32) -> Result<Person, Error> {
            let request_url = format!("{}/{}", url.to_string(), id);
            let response = reqwest::get(&request_url).await?;
            let p: Person = response.json().await?;

            Ok(p)
        }

        match get_person_http((&self.url).to_string(), id) {
            Ok(person) => person,
            Err(e) => panic!{"{:?}", e}
        }
    }

    pub fn get_people(&self) -> Value {
        #[tokio::main]
        async fn get_people_http(url: String) -> Result<Value, Error> {
            let response = reqwest::get(&url.to_string()).await?;
            let p: Value = response.json().await?;

            Ok(p)
        }

        match get_people_http((&self.url).to_string()) {
            Ok(people) => people,
            Err(e) => panic!{"{:?}", e}
        }
    }
}