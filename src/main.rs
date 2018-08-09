use std::{thread, time};
use std::io::{Write, stdout};

#[macro_use]
extern crate clap;
use clap::{Arg, App};
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termion;
use termion::input::{MouseTerminal};


mod common;
use common::{BBox, Point};
mod services;
use services::postcodes::{PostcodeLocation};

/// Returns the clap command line parser
fn main_parser<'a>() -> clap::App<'a, 'a> {
    App::new("flights-overhead")
        .arg(Arg::with_name("POSTCODE")
            .help("Postcode to center search area (UK)")
            .required(true)
            .index(1))
        .arg(Arg::with_name("RADIUS")
            .help("Radius of the area to monitor (km)")
            .index(2))
}

/// Get the equal or next lowest odd number
fn lower_odd(i: u64) -> u64 {
    (i - 1) | 1
}

/// Get the midpoint of an odd number
fn origin_odd(i: u64) -> u64 {
    (i + 1) / 2
}

/// main function
fn main() {
    // Cli parser
    let matches = main_parser().get_matches();
    let postcode: &str = matches.value_of("POSTCODE").unwrap();
    let radius: f64 = value_t!(matches, "RADIUS", f64).unwrap_or(1.0);

    // Get our position and surrounding area
    let location = PostcodeLocation::from_postcode(&postcode);
    let bbox = BBox::surrounding(Point{ lng: location.longitude, lat: location.latitude }, radius);
    
    // Refresh rate
    let tick = time::Duration::from_millis(8000);

    // Drawing handle
    let mut stdout = MouseTerminal::from(stdout());

    loop {
        // Calculate drawing constants for this frame
        let term_size = termion::terminal_size().unwrap_or((24, 80));
        let canvas_size = (lower_odd(term_size.0 as u64), lower_odd(term_size.1 as u64));
        let origin = (origin_odd(canvas_size.0), origin_odd(canvas_size.1));

        // Ping API, get live data
        let states = services::opensky::flights_over(&bbox);

        // Clear screen and mark location
        write!(stdout, "{}", termion::clear::All).unwrap();
        write!(stdout, "{}x", termion::cursor::Goto(origin.0 as u16, origin.1 as u16)).unwrap();

        // For each plane
        for state in &states {
            match &state.position {
                // If we know the position, draw it
                Some(p) => {
                    // Find offsets from origin
                    let x_delta = (((p.lng - location.longitude) / bbox.lng_radius) * origin.0 as f64).floor() as i64;
                    let y_delta = (((p.lat - location.latitude) / bbox.lat_radius) * origin.1 as f64).floor() as i64;
                    // Convert to terminal coordinates
                    let x = origin.0 as i64 + x_delta;
                    let y = origin.1 as i64 - y_delta;
                    // Draw
                    write!(stdout, "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();
                    write!(stdout, ".{}", state.callsign).unwrap();
                },
                None => ()
            };
        }

        // Stop drawing, flush the buffer and sleep
        write!(stdout, "{}", termion::cursor::Goto(canvas_size.0 as u16, canvas_size.1 as u16)).unwrap();
        stdout.flush().unwrap();
        thread::sleep(tick);
    }
}

