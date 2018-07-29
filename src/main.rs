extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

mod services;
use services::postcodes::{PostcodeLocation};

#[derive(Debug)]
struct BBox {
    pub lng_min: f64,
    pub lng_max: f64,
    pub lat_min: f64,
    pub lat_max: f64
}

impl BBox {
    pub fn surrounding(point: Point, diameter: f64) -> BBox {
        let delta_lng = (diameter / (111.320 * point.lat.to_radians().cos())) / 2.0;
        let delta_lat = (diameter / 110.574) / 2.0;

        BBox {
            lng_min: point.lng - delta_lng,
            lng_max: point.lng + delta_lng,
            lat_min: point.lat - delta_lat,
            lat_max: point.lat + delta_lat
        }
    }
}

#[derive(Debug)]
struct Point {
    pub lng: f64,
    pub lat: f64,
}


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
    println!("location: {:?}", bbox);

}

