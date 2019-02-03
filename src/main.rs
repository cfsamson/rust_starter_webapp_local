use chrono;
use handlebars;
use rouille;
use serde;

use chrono::Local;

mod data;
mod web;

fn main() -> Res<()> {
    // -------- INITIALIZATION --------
    data::create()?;

    // -------- START WEBSERVER --------
    web::start(Some(8080))?;

    Ok(())
}

const MAX_PORT: u32 = 8090;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;


