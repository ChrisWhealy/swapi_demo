use crate::sort::compare_strs_as_f64s;
use crate::{
    sort::{compare_strs, Sorter},
    SwapiResponse, SwapiType,
};
use reqwest::Error;
use crate::macros::gen_fetch_fn;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Person {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub hair_color: String,
    pub skin_color: String,
    pub eye_color: String,
    pub birth_year: String,
    pub gender: String,
    #[serde(rename = "homeworld")]
    pub home_world: String,
    pub films: Vec<String>,
    pub species: Vec<String>,
    pub vehicles: Vec<String>,
    pub starships: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Person {}

impl Person {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Person> {
        match (field, order) {
            ("height", "desc") => |s1: &Person, s2: &Person| compare_strs_as_f64s(&s1.height, &s2.height).reverse(),
            ("height", "asc") => |s1: &Person, s2: &Person| compare_strs_as_f64s(&s1.height, &s2.height),
            ("mass", "desc") => |s1: &Person, s2: &Person| compare_strs_as_f64s(&s1.mass, &s2.mass).reverse(),
            ("mass", "asc") => |s1: &Person, s2: &Person| compare_strs_as_f64s(&s1.mass, &s2.mass),

            // Default to sorting by name
            (_, "desc") => |s1: &Person, s2: &Person| compare_strs(&s1.name, &s2.name).reverse(),
            (_, _) => |s1: &Person, s2: &Person| compare_strs(&s1.name, &s2.name)
        }
    }
}

gen_fetch_fn!("people", Person);
