use crate::{
    macros::gen_fetch_fn,
    sort::{compare_strs, compare_strs_as_f64s, Sorter}, SwapiResponse,
    SwapiType,
};
use reqwest::Error;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Planet {
    pub name: String,
    pub rotation_period: String,
    pub orbital_period: String,
    pub diameter: String,
    pub climate: String,
    pub gravity: String,
    pub terrain: String,
    pub surface_water: String,
    pub population: String,
    pub residents: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Planet {}

impl Planet {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Planet> {
        match (field, order) {
            ("diameter", "desc") => |s1: &Planet, s2: &Planet| {
                compare_strs_as_f64s(&s1.diameter, &s2.diameter).reverse()
            },
            ("diameter", "asc") => {
                |s1: &Planet, s2: &Planet| compare_strs_as_f64s(&s1.diameter, &s2.diameter)
            }
            ("population", "desc") => |s1: &Planet, s2: &Planet| {
                compare_strs_as_f64s(&s1.population, &s2.population).reverse()
            },
            ("population", "asc") => {
                |s1: &Planet, s2: &Planet| compare_strs_as_f64s(&s1.population, &s2.population)
            }

            // Default to sorting by name
            (_, "desc") => |s1: &Planet, s2: &Planet| compare_strs(&s1.name, &s2.name).reverse(),
            (_, _) => |s1: &Planet, s2: &Planet| compare_strs(&s1.name, &s2.name),
        }
    }
}

gen_fetch_fn!("planets", Planet);
