#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub skin_color: String,
    pub vehicles: Vec<String>,
    pub created: String
}