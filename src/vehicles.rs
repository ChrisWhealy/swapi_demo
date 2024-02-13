use crate::{
    utils::{compare_strs, compare_strs_as_f64s},
    SwapiType,
    gen_str_sort_fn,
    gen_num_str_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_VEHICLES: &str = include_str!("./data/vehicles.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vehicle {
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    #[serde(rename = "cost_in_credits")]
    pub cost: String,
    pub length: String,
    pub max_atmosphering_speed: String,  //Yes, "atmosphering" is not a real word...
    pub crew: String,
    pub passengers: String,
    pub cargo_capacity: String,
    pub consumables: String,
    pub vehicle_class: String,
    pub pilots: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Vehicle {}

impl Vehicle {
    gen_str_sort_fn!(name, Vehicle);
    gen_str_sort_fn!(manufacturer, Vehicle);
    gen_num_str_sort_fn!(length, Vehicle);
    gen_num_str_sort_fn!(cost, Vehicle);
}
