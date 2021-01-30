#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod config;
mod routes;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;
use dotenv::dotenv;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "message": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
  dotenv().ok();
  rocket::custom(config::from_env())
    .mount("/api", routes![
      routes::url_shortener::get_url,
    ])
    .attach(cors_fairing())
    .register(catchers![not_found])
}