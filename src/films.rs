use crate::{
    macros::gen_fetch_fn,
    sort::{compare_strs, Sorter},
    SwapiResponse, SwapiType,
};
use reqwest::Error;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Film {
    pub title: String,
    pub episode_id: u16,
    pub opening_crawl: String,
    pub director: String,
    pub producer: String,
    pub release_date: String,
    pub characters: Vec<String>,
    pub planets: Vec<String>,
    pub starships: Vec<String>,
    pub vehicles: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Film {}

impl Film {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Film> {
        match (field, order) {
            ("title", "desc") => {
                |s1: &Film, s2: &Film| compare_strs(&s1.title, &s2.title).reverse()
            }
            ("title", "asc") => |s1: &Film, s2: &Film| compare_strs(&s1.title, &s2.title),
            ("release_date", "desc") => {
                |s1: &Film, s2: &Film| compare_strs(&s1.release_date, &s2.release_date).reverse()
            }
            ("release_date", "asc") => {
                |s1: &Film, s2: &Film| compare_strs(&s1.release_date, &s2.release_date)
            }

            // Default to sorting by episode_id
            (_, "desc") => |s1: &Film, s2: &Film| s1.episode_id.cmp(&s2.episode_id).reverse(),
            (_, _) => |s1: &Film, s2: &Film| s1.episode_id.cmp(&s2.episode_id),
        }
    }
}

gen_fetch_fn!("films", Film);
