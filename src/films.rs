use crate::{
    utils::{compare_strs, compare_nums},
    SwapiType,
    gen_str_sort_fn,
    gen_num_sort_fn
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub static SWAPI_FILMS: &str = include_str!("./data/films.json");

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Film {
    pub title: String,
    #[serde(rename = "episode_id")]
    pub episode: u32,
    pub opening_crawl: String,
    pub director: String,
    pub producer: String,
    pub release_date: String,
    pub characters: Vec<String>,
    pub species: Vec<String>,
    pub vehicles: Vec<String>,
    pub starships: Vec<String>,
    pub planets: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Film {}

impl Film {
    gen_str_sort_fn!(title, Film);
    gen_str_sort_fn!(director, Film);
    gen_num_sort_fn!(episode, Film);
}
