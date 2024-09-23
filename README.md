# REST-based API into Replicated Star Wars Data

The <https://swapi.py4e.com> website exposes a variety of JSON-based information about the Star Wars films:

* Films
* People
* Planets
* Species
* Starships
* vehicles

This app starts a server that exposes the same information by fetching it from the above server then storing it in a runtime cache. 

## Execution

Clone this repo into a local folder then start the service using `cargo run`.

You can now access the data via <http://127.0.0.1:3000>
