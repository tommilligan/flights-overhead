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
            .help("Postcode to center search area")
            .required(true)
            .index(1))
        .arg(Arg::with_name("RADIUS")
            .help("Radius of the area to monitor")
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
    let matches = main_parser().get_matches();

    let term_size = termion::terminal_size().unwrap_or((24, 80));
    let canvas_size = (lower_odd(term_size.0 as u64), lower_odd(term_size.1 as u64));
    let origin = (origin_odd(canvas_size.0), origin_odd(canvas_size.1));

    let postcode: &str = matches.value_of("POSTCODE").unwrap();
    let radius: f64 = value_t!(matches, "RADIUS", f64).unwrap_or(1.0);

    let location = PostcodeLocation::from_postcode(&postcode);
    let bbox = BBox::surrounding(Point{ lng: location.longitude, lat: location.latitude }, radius);
    
    let tick = time::Duration::from_millis(10000);

    let mut stdout = MouseTerminal::from(stdout());

    loop {
        let states = services::opensky::flights_over(&bbox);

        if false {
            println!("Currently overhead:");
            for state in &states {
                println!("- {}: {}", state.callsign, match &state.position {
                    Some(p) => format!("{}", p),
                    None => String::from("(position unknown)")
                });
            }
        } else {
            // write!(stdout, "{}", termion::clear::All).unwrap();
            for state in &states {
                match &state.position {
                    Some(p) => {
                        let x = ((p.lng - location.longitude) / radius).floor() as u64 + origin.0;
                        let y = ((p.lat - location.latitude) / radius).floor() as u64 + origin.1;
                        write!(stdout, "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();
                        write!(stdout, "{}", state.callsign).unwrap();
                    },
                    None => ()
                };
            }
            stdout.flush().unwrap();
        }

        thread::sleep(tick);
    }
}

