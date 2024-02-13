use crate::{
    utils::{compare_strs, compare_strs_as_f64s},
    SwapiType,
    gen_str_sort_fn,
    gen_num_str_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_PEOPLE: &str = include_str!("./data/people.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Person {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub hair_color: String,
    pub skin_color: String,
    pub eye_color: String,
    pub birth_year: String,
    pub gender: String,
    pub homeworld: String,
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
    gen_str_sort_fn!(name, Person);
    gen_str_sort_fn!(homeworld, Person);
    gen_num_str_sort_fn!(height, Person);
}
