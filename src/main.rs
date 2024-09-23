mod films;
mod people;
mod planets;
mod sort;
mod species;
mod starships;
mod vehicles;
mod macros;

use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

use films::*;
use people::*;
use planets::*;
use starships::*;
use species::*;
use vehicles::*;
use macros::*;

const SELF_HOSTNAME: &str = "127.0.0.1";
const SELF_PORT: u16 = 3000;
const API_URL: &str = "https://swapi.py4e.com/api/";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait SwapiType {}

#[derive(serde::Deserialize)]
pub struct SwapiResponse<T>
where
    T: SwapiType,
{
    count: u32,
    next: Option<String>,
    #[serde(rename = "previous")]
    _previous: Option<String>,
    results: Vec<T>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Clone, serde::Deserialize)]
struct QueryString {
    #[serde(rename = "sortBy")]
    sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    sort_order: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Mutable shared cache
pub struct SwapiCache {
    pub films: Mutex<Vec<Film>>,
    pub species: Mutex<Vec<Species>>,
    pub starships: Mutex<Vec<Starship>>,
    pub people: Mutex<Vec<Person>>,
    pub planets: Mutex<Vec<Planet>>,
    pub vehicles: Mutex<Vec<Vehicle>>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("./assets/usage.html"))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate handler functions
gen_handler_fn!("films", "episode_id", Film, "/films");
gen_handler_fn!("people", "name", Person, "/people");
gen_handler_fn!("planets", "name", Planet, "/planets");
gen_handler_fn!("species", "name", Species, "/species");
gen_handler_fn!("starships", "name", Starship, "/starships");
gen_handler_fn!("vehicles", "name", Vehicle, "/vehicles");

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache = web::Data::new(SwapiCache {
        films: Mutex::new(vec![]),
        species: Mutex::new(vec![]),
        starships: Mutex::new(vec![]),
        people: Mutex::new(vec![]),
        planets: Mutex::new(vec![]),
        vehicles: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(cache.clone())
            .service(get_index)
            .service(handle_films)
            .service(handle_people)
            .service(handle_planets)
            .service(handle_species)
            .service(handle_starships)
            .service(handle_vehicles)
    })
        .bind((SELF_HOSTNAME, SELF_PORT))?
        .run()
        .await
}
