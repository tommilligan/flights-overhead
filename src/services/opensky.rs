extern crate reqwest;

use self::super::super::common::{BBox, Point};

#[derive(Serialize, Deserialize, Debug)]
struct OpenSkyApiResponse {
    time: u64,
    states: Option<Vec<OpenSkyState>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenSkyState(
    String,
    Option<String>,
    String,
    Option<i64>,
    i64,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    bool,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<Vec<i64>>,
    Option<f64>,
    Option<String>,
    bool,
    i64
);

#[derive(Debug)]
pub struct Plane {
    pub icao24: String,
    pub callsign: String,
    pub origin_country: String,
    pub time_position: Option<i64>,
    pub last_contact: i64,
    pub position: Option<Point>,
    pub geo_altitude: Option<f64>,
    pub on_ground: bool,
    pub velocity: Option<f64>,
    pub true_track: Option<f64>,
    pub vertical_rate: Option<f64>,
    pub sensors: Option<Vec<i64>>,
    pub baro_altitude: Option<f64>,
    pub squawk: Option<String>,
    pub spi: bool,
    pub position_source: i64
}

impl Plane {
    pub fn from_state<'a>(state: OpenSkyState) -> Plane {
        Plane {
            icao24: state.0,
            callsign: match state.1 {
                Some(s) => s.trim().to_string(),
                None => String::from("")
            },
            origin_country: state.2,
            time_position: state.3,
            last_contact: state.4,
            position: match state.5 {
                Some(lng) => match state.6 {
                    Some(lat) => Some(Point {lng: lng, lat: lat}),
                    None => None
                },
                None => None
            },
            geo_altitude: state.7,
            on_ground: state.8,
            velocity: state.9,
            true_track: state.10,
            vertical_rate: state.11,
            sensors: state.12,
            baro_altitude: state.13,
            squawk: state.14,
            spi: state.15,
            position_source: state.16
        }
    }
}

pub fn flights_over(bbox: BBox) -> Vec<Plane> {
    let opensky_url = format!(
        "https://opensky-network.org/api/states/all?lomin={}&lamin={}&lomax={}&lamax={}",
        bbox.lng_min,
        bbox.lat_min,
        bbox.lng_max,
        bbox.lat_max
    );
    let res: OpenSkyApiResponse = reqwest::get(&opensky_url).unwrap().json().unwrap();
    match res.states {
        Some(states) => states.into_iter().map(Plane::from_state).collect(), 
        None => vec![]
    }
}
