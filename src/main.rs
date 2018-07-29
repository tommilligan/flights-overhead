extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
use clap::{Arg, App};

mod services;
use services::postcode::{PostcodeLocation};

/// Returns the clap command line parser
fn main_parser<'a>() -> clap::App<'a, 'a> {
    App::new("flights-overhead")
        .arg(Arg::with_name("POSTCODE")
            .help("Postcode to center search area")
            .required(true)
            .index(1))
}

/// main function
fn main() {
    let matches = main_parser().get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let postcode: &str = matches.value_of("POSTCODE").unwrap();

    let location = PostcodeLocation::from_postcode(&postcode);

}

