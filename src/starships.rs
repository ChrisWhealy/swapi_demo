use crate::sort::compare_strs_as_f64s;
use crate::{
    sort::{compare_strs, Sorter},
    SwapiResponse, SwapiType,
};
use reqwest::Error;
use crate::macros::gen_fetch_fn;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Starship {
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    #[serde(rename = "cost_in_credits")]
    pub cost: String,
    pub length: String,
    pub max_atmosphering_speed: String,
    pub crew: String,
    pub passengers: String,
    pub cargo_capacity: String,
    pub consumables: String,
    pub hyperdrive_rating: String,
    #[serde(rename = "MGLT")]
    pub mglt: String,
    pub starship_class: String,
    pub pilots: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Starship {}

impl Starship {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Starship> {
        match (field, order) {
            ("length", "desc") => |s1: &Starship, s2: &Starship| compare_strs_as_f64s(&s1.length, &s2.length).reverse(),
            ("length", "asc") => |s1: &Starship, s2: &Starship| compare_strs_as_f64s(&s1.length, &s2.length),
            ("cost", "desc") => |s1: &Starship, s2: &Starship| compare_strs_as_f64s(&s1.cost, &s2.cost).reverse(),
            ("cost", "asc") => |s1: &Starship, s2: &Starship| compare_strs_as_f64s(&s1.cost, &s2.cost),

            // Default to sorting by name
            (_, "desc") => |s1: &Starship, s2: &Starship| compare_strs(&s1.name, &s2.name).reverse(),
            (_, _) => |s1: &Starship, s2: &Starship| compare_strs(&s1.name, &s2.name)
        }
    }
}

gen_fetch_fn!("starships", Starship);
