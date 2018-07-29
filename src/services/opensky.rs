extern crate reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct OpenSkyApiResponse {
    time: u64,
    states: Option<Vec<OpenSkyState>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenSkyState(
    String,
    String,
    String,
    i64,
    i64,
    f64,
    f64,
    f64,
    bool,
    f64,
    f64,
    f64,
    Option<Vec<i64>>,
    f64,
    String,
    bool,
    i64
);

