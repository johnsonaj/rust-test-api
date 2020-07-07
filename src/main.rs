#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate env_logger;


use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod swapi;
use swapi::people::PeopleService;
use swapi::vehicles::VehicleService;

#[get("/")]
fn get_people(ps: State<PeopleService>) -> Json<JsonValue> {
    let p = ps.get_people();

    Json(json!(p))
}

#[get("/<id>")]
fn get_person(id: i32, ps: State<PeopleService>) -> Json<JsonValue> {
    let p = ps.get_person(id);

    Json(json!(p))
}

#[get("/")]
fn get_vehicles(vs: State<VehicleService>) -> Json<JsonValue> {
    let v = vs.get_vehicles();

    Json(json!(v))
}

#[get("/<id>")]
fn get_vehicle(id: i32, vs: State<VehicleService>) -> Json<JsonValue> {
    let v = vs.get_vehicle(id);

    Json(json!(v))
}


fn main() {
    //env_logger::init();
    let base_url = "https://swapi.dev/api/";
    let vs = VehicleService::new(base_url);
    let ps = PeopleService::new(base_url);    

    rocket::ignite()
        .manage(ps)
        .manage(vs)
        .mount("/people", routes![get_person, get_people])
        .mount("/vehicles", routes![get_vehicle, get_vehicles])
        .launch();
}