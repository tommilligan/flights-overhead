use std::{thread, time};

#[macro_use]
extern crate clap;
use clap::{Arg, App};
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod common;
mod services;
use services::postcodes::{PostcodeLocation};
use common::{BBox, Point};

/// Returns the clap command line parser
fn main_parser<'a>() -> clap::App<'a, 'a> {
    App::new("flights-overhead")
        .arg(Arg::with_name("POSTCODE")
            .help("Postcode to center search area")
            .required(true)
            .index(1))
        .arg(Arg::with_name("DIAMETER")
            .help("Diameter of the area to monitor")
            .index(2))
}

/// main function
fn main() {
    let matches = main_parser().get_matches();

    let postcode: &str = matches.value_of("POSTCODE").unwrap();
    let diameter: f64 = value_t!(matches, "DIAMETER", f64).unwrap_or(1.0);

    let location = PostcodeLocation::from_postcode(&postcode);
    let bbox = BBox::surrounding(Point{ lng: location.longitude, lat: location.latitude }, diameter);
    
    let tick = time::Duration::from_millis(30000);
    loop {
        let states = services::opensky::flights_over(&bbox);

        println!("Currently overhead:");
        for state in &states {
            println!("- {}: {}", state.callsign, match &state.position {
                Some(p) => format!("{}", p),
                None => String::from("(position unknown)")
            });
        }

        thread::sleep(tick);
    }
}

