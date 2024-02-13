use crate::{
    utils::{compare_strs, compare_strs_as_f64s},
    SwapiType,
    gen_str_sort_fn,
    gen_num_str_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_STARSHIPS: &str = include_str!("./data/starships.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
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
    gen_str_sort_fn!(name, Starship);
    gen_num_str_sort_fn!(cost, Starship);
    gen_num_str_sort_fn!(length, Starship);
}
