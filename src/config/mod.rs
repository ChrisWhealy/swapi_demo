use crate::films::*;
use crate::people::*;
use crate::planets::*;
use crate::species::*;
use crate::starships::*;
use crate::vehicles::*;

use std::cmp::Ordering;
use std::collections::HashMap;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Sort configuration
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct Config<'a> {
    pub films: HashMap<&'a str, fn(&Film, &Film) -> Ordering>,
    pub people: HashMap<&'a str, fn(&Person, &Person) -> Ordering>,
    pub planets: HashMap<&'a str, fn(&Planet, &Planet) -> Ordering>,
    pub species: HashMap<&'a str, fn(&Species, &Species) -> Ordering>,
    pub starships: HashMap<&'a str, fn(&Starship, &Starship) -> Ordering>,
    pub vehicles: HashMap<&'a str, fn(&Vehicle, &Vehicle) -> Ordering>,
}

pub fn get_config<'a>() -> Config<'a> {
    let mut config_film = HashMap::new();

    config_film.insert("title.asc", Film::sort_by_title("asc"));
    config_film.insert("title.desc", Film::sort_by_title("desc"));
    config_film.insert("episode.asc", Film::sort_by_episode("asc"));
    config_film.insert("episode.desc", Film::sort_by_episode("desc"));
    config_film.insert("director.asc", Film::sort_by_director("asc"));
    config_film.insert("director.desc", Film::sort_by_director("desc"));

    let mut config_people = HashMap::new();

    config_people.insert("name.asc", Person::sort_by_name("asc"));
    config_people.insert("name.desc", Person::sort_by_name("desc"));
    config_people.insert("height.asc", Person::sort_by_height("asc"));
    config_people.insert("height.desc", Person::sort_by_height("desc"));
    config_people.insert("homeworld.asc", Person::sort_by_homeworld("asc"));
    config_people.insert("homeworld.desc", Person::sort_by_homeworld("desc"));

    let mut config_planets = HashMap::new();

    config_planets.insert("name.asc", Planet::sort_by_name("asc"));
    config_planets.insert("name.desc", Planet::sort_by_name("desc"));
    config_planets.insert("rotation.asc", Planet::sort_by_rotation("asc"));
    config_planets.insert("rotation.desc", Planet::sort_by_rotation("desc"));
    config_planets.insert("orbit.asc", Planet::sort_by_orbit("asc"));
    config_planets.insert("orbit.desc", Planet::sort_by_orbit("desc"));

    let mut config_species = HashMap::new();

    config_species.insert("name.asc", Species::sort_by_name("asc"));
    config_species.insert("name.desc", Species::sort_by_name("desc"));
    config_species.insert("height.asc", Species::sort_by_height("asc"));
    config_species.insert("height.desc", Species::sort_by_height("desc"));
    config_species.insert("lifespan.asc", Species::sort_by_lifespan("asc"));
    config_species.insert("lifespan.desc", Species::sort_by_lifespan("desc"));
    config_species.insert("language.asc", Species::sort_by_language("asc"));
    config_species.insert("language.desc", Species::sort_by_language("desc"));

    let mut config_starships = HashMap::new();

    config_starships.insert("name.asc", Starship::sort_by_name("asc"));
    config_starships.insert("name.desc", Starship::sort_by_name("desc"));
    config_starships.insert("length.asc", Starship::sort_by_length("asc"));
    config_starships.insert("length.desc", Starship::sort_by_length("desc"));
    config_starships.insert("cost.asc", Starship::sort_by_cost("asc"));
    config_starships.insert("cost.desc", Starship::sort_by_cost("desc"));

    let mut config_vehicles = HashMap::new();

    config_vehicles.insert("name.asc", Vehicle::sort_by_name("asc"));
    config_vehicles.insert("name.desc", Vehicle::sort_by_name("desc"));
    config_vehicles.insert("manufacturer.asc", Vehicle::sort_by_manufacturer("asc"));
    config_vehicles.insert("manufacturer.desc", Vehicle::sort_by_manufacturer("desc"));
    config_vehicles.insert("length.asc", Vehicle::sort_by_length("asc"));
    config_vehicles.insert("length.desc", Vehicle::sort_by_length("desc"));
    config_vehicles.insert("cost.asc", Vehicle::sort_by_cost("asc"));
    config_vehicles.insert("cost.desc", Vehicle::sort_by_cost("desc"));

    Config {
        films: config_film,
        people: config_people,
        planets: config_planets,
        species: config_species,
        starships: config_starships,
        vehicles: config_vehicles,
    }
}
