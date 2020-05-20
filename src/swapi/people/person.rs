extern crate reqwest;
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub skin_color: String,
    pub vehicles: Vehicles,
    pub created: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonExternal {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub skin_color: String,
    pub vehicles: Vec<Vehicle>,
    pub created: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub length: String,
    pub crew: String,
    pub cargo_capacity: String
}

type Vehicles = Vec<String>;

pub trait Find {
    fn find(&self) -> Vec<Vehicle>;
}

impl Find for Vehicles {
    fn find(&self) -> Vec<Vehicle> {
        let mut vehicle_list = Vec::new();
        for v in self {
            let ve = get_vehicle(v);
            vehicle_list.push(ve);
        }
        vehicle_list
    }
}

fn get_vehicle(url: &str) -> Vehicle {
    #[tokio::main]
    async fn get_vehicle(url: &str)  -> Result<Vehicle, Error> {
        let response = reqwest::get(&url.to_string()).await?;
        let v: Vehicle = response.json().await?;
        Ok(v)
    }

    match get_vehicle(url) {
        Ok(vehicle) => vehicle,
        Err(e) => panic!("{:?}", e)
    }
}