extern crate clap;
extern crate reqwest;
use clap::{Arg, App};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct PostcodeApiResponse<T> {
    status: u64,
    result: T
}

#[derive(Serialize, Deserialize, Debug)]
struct PostcodeLocation {
    country: String,
    latitude: f64,
    longitude: f64,
    postcode: String,
    region: String
}

/// Get location metadata from a postcode
fn get_postcode_location(postcode: &str) -> PostcodeLocation {
    let postcode_url = format!("https://api.postcodes.io/postcodes/{}", postcode);
    let res: PostcodeApiResponse<PostcodeLocation> = reqwest::get(&postcode_url).unwrap().json().unwrap();
    println!("response: {:?}", res);
    res.result
}

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

    let location = get_postcode_location(&postcode);

}

