use crate::{
    sort::{compare_strs, Sorter},
    SwapiResponse, SwapiType,
};
use reqwest::Error;
use crate::sort::compare_strs_as_f64s;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Vehicle {
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
    pub vehicle_class: String,
    pub pilots: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

impl SwapiType for Vehicle {}

impl Vehicle {
    pub fn sort_by(field: &str, order: &str) -> Sorter<Vehicle> {
        match (field, order) {
            ("cost", "desc") => |s1: &Vehicle, s2: &Vehicle| compare_strs_as_f64s(&s1.cost, &s2.cost).reverse(),
            ("cost", "asc") => |s1: &Vehicle, s2: &Vehicle| compare_strs_as_f64s(&s1.cost, &s2.cost),
            ("length", "desc") => |s1: &Vehicle, s2: &Vehicle| compare_strs_as_f64s(&s1.length, &s2.length).reverse(),
            ("length", "asc") => |s1: &Vehicle, s2: &Vehicle| compare_strs_as_f64s(&s1.length, &s2.length),

            // Default to sorting by name
            (_, "desc") => |s1: &Vehicle, s2: &Vehicle| compare_strs(&s1.name, &s2.name).reverse(),
            (_, _) => |s1: &Vehicle, s2: &Vehicle| compare_strs(&s1.name, &s2.name),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub async fn fetch_vehicles(url: &str) -> Result<Option<SwapiResponse<Vehicle>>, Error> {
    let mut page = reqwest::get(url)
        .await?
        .json::<SwapiResponse<Vehicle>>()
        .await?;

    let mut response: SwapiResponse<Vehicle> = SwapiResponse::<Vehicle> {
        count: page.count,
        next: None,
        _previous: None,
        results: page.results,
    };

    while page.next.is_some() {
        page = reqwest::get(page.next.clone().unwrap())
            .await?
            .json::<SwapiResponse<Vehicle>>()
            .await?;

        response.results.append(&mut page.results)
    }

    Ok(Some(response))
}
