#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub length: String,
    pub crew: String,
    pub cargo_capacity: String
}