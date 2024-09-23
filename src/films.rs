use crate::{
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
            ("title", "desc") => |s1: &Film, s2: &Film| compare_strs(&s1.title, &s2.title).reverse(),
            ("title", "asc") => |s1: &Film, s2: &Film| compare_strs(&s1.title, &s2.title),
            ("release_date", "desc") => |s1: &Film, s2: &Film| compare_strs(&s1.release_date, &s2.release_date).reverse(),
            ("release_date", "asc") => |s1: &Film, s2: &Film| compare_strs(&s1.release_date, &s2.release_date),

            // Default to sorting by episode_id
            (_, "desc") => |s1: &Film, s2: &Film| s1.episode_id.cmp(&s2.episode_id).reverse(),
            (_, _) => |s1: &Film, s2: &Film| s1.episode_id.cmp(&s2.episode_id),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn fetch_films(url: &str) -> Result<Option<SwapiResponse<Film>>, Error> {
    let mut page = reqwest::get(url)
        .await?
        .json::<SwapiResponse<Film>>()
        .await?;

    let mut response: SwapiResponse<Film> = SwapiResponse::<Film> {
        count: page.count,
        next: None,
        _previous: None,
        results: page.results,
    };

    while page.next.is_some() {
        page = reqwest::get(page.next.clone().unwrap())
            .await?
            .json::<SwapiResponse<Film>>()
            .await?;

        response.results.append(&mut page.results)
    }

    Ok(Some(response))
}
