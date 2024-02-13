use crate::{
    utils::{compare_strs, compare_strs_as_f64s},
    SwapiType,
    gen_str_sort_fn,
    gen_num_str_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_SPECIES: &str = include_str!("./data/species.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Species {
    pub name: String,
    pub classification: String,
    pub designation: String,
    #[serde(rename = "average_height")]
    pub height: String,
    pub skin_colors: String,
    pub hair_colors: String,
    pub eye_colors: String,
    #[serde(rename = "average_lifespan")]
    pub lifespan: String,
    pub homeworld: Option<String>,
    pub language: String,
    pub people: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Species {}

impl Species {
    gen_str_sort_fn!(name, Species);
    gen_str_sort_fn!(language, Species);
    gen_num_str_sort_fn!(height, Species);
    gen_num_str_sort_fn!(lifespan, Species);
}
