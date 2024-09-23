use crate::{
    sort::{compare_strs, Sorter},
    SwapiResponse, SwapiType,
};
use reqwest::Error;
use crate::macros::gen_fetch_fn;
use crate::sort::compare_strs_as_f64s;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Species {
    pub name: String,
    pub classification: String,
    pub designation: String,
    pub average_height: String,
    pub skin_colors: String,
    pub hair_colors: String,
    pub average_lifespan: String,
    #[serde(rename = "homeworld")]
    pub home_world: Option<String>,
    pub language: String,
    pub people: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Species {}

impl Species {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Species> {
        match (field, order) {
            ("height", "desc") => |s1: &Species, s2: &Species| compare_strs_as_f64s(&s1.average_height, &s2.average_height).reverse(),
            ("height", "asc") => |s1: &Species, s2: &Species| compare_strs_as_f64s(&s1.average_height, &s2.average_height),
            ("lifespan", "desc") => |s1: &Species, s2: &Species| compare_strs_as_f64s(&s1.average_lifespan, &s2.average_lifespan).reverse(),
            ("lifespan", "asc") => |s1: &Species, s2: &Species| compare_strs_as_f64s(&s1.average_lifespan, &s2.average_lifespan),

            // Default to sorting by name
            (_, "desc") => |s1: &Species, s2: &Species| compare_strs(&s1.name, &s2.name).reverse(),
            (_, _) => |s1: &Species, s2: &Species| compare_strs(&s1.name, &s2.name),
        }
    }
}

gen_fetch_fn!("species", Species);

