mod config;
mod films;
mod people;
mod planets;
mod species;
mod starships;
mod vehicles;

mod utils;

use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use films::*;
use people::*;
use planets::*;
use species::*;
use starships::*;
use vehicles::*;

use config::*;
use utils::*;

pub trait SwapiType {}

#[derive(Deserialize)]
pub struct SwapiResponse<T>
where
    T: SwapiType,
{
    #[serde(rename = "count")]
    _count: u32,
    #[serde(rename = "next")]
    _next: Option<String>,
    #[serde(rename = "previous")]
    _previous: Option<String>,
    pub results: Vec<T>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Root path handler
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("./data/usage.html"))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate path handlers
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
gen_handler_fn!("films", "title", Film, SWAPI_FILMS, "/films");
gen_handler_fn!("people", "name", Person, SWAPI_PEOPLE, "/people");
gen_handler_fn!("planets", "name", Planet, SWAPI_PLANETS, "/planets");
gen_handler_fn!("species", "name", Species, SWAPI_SPECIES, "/species");
gen_handler_fn!("starships", "name", Starship, SWAPI_STARSHIPS, "/starships");
gen_handler_fn!("vehicles", "name", Vehicle, SWAPI_VEHICLES, "/vehicles");

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Start server
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(get_config()))
            .service(get_index)
            .service(get_films)
            .service(get_people)
            .service(get_planets)
            .service(get_species)
            .service(get_starships)
            .service(get_vehicles)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
