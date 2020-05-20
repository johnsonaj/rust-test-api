extern crate reqwest;
use reqwest::Error;
use serde_json::Value;

mod person;
use person::{Person, PersonExternal, Find};

pub struct PeopleService {
    url: &'static str
}

impl PeopleService {
    pub fn new(base: &'static str) -> PeopleService {
        PeopleService{
            url: base
        }
    }

    pub fn get_person(&self, id: i32) -> PersonExternal {
        let p = &self.get_person_internal(id);
        let pe = PersonExternal {
            name: p.name.to_owned(),
            height: p.height.to_owned(),
            mass: p.mass.to_owned(),
            skin_color: p.skin_color.to_owned(),
            vehicles: p.vehicles.find(),
            created: p.created.to_owned()
        };
        
        pe
    }

    pub fn get_people(&self) -> Value {
        #[tokio::main]
        async fn get_people_http(url: String) -> Result<Value, Error> {
            let request_url = format!("{}people", &url.to_string());
            let response = reqwest::get(&request_url).await?;
            let p: Value = response.json().await?;

            Ok(p)
        }

        match get_people_http((&self.url).to_string()) {
            Ok(people) => people,
            Err(e) => panic!{"{:?}", e}
        }
    }

    fn get_person_internal(&self, id: i32) -> Person {
        #[tokio::main]
        async fn get_person_http(url: String, id: i32) -> Result<Person, Error> {
            let request_url = format!("{}people/{}", url.to_string(), id);
            let response = reqwest::get(&request_url).await?;
            let p: Person = response.json().await?;

            Ok(p)
        }

        match get_person_http((&self.url).to_string(), id) {
            Ok(person) => person,
            Err(e) => panic!{"{:?}", e}
        }
    }
}