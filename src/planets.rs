use crate::{
    utils::{compare_strs, compare_strs_as_f64s},
    SwapiType,
    gen_str_sort_fn,
    gen_num_str_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_PLANETS: &str = include_str!("./data/planets.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Planet {
    pub name: String,
    #[serde(rename = "rotation_period")]
    pub rotation: String,
    #[serde(rename = "orbital_period")]
    pub orbit: String,
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
    gen_str_sort_fn!(name, Planet);
    gen_num_str_sort_fn!(rotation, Planet);
    gen_num_str_sort_fn!(orbit, Planet);
}
