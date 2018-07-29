extern crate reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct PostcodeApiResponse<T> {
    status: u64,
    result: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostcodeLocation {
    country: String,
    latitude: f64,
    longitude: f64,
    postcode: String,
    region: String
}

// Implementation block, all `Point` methods go in here
impl PostcodeLocation {
    /// Get location metadata from a postcode
    pub fn from_postcode(postcode: &str) -> PostcodeLocation {
        let postcode_url = format!("https://api.postcodes.io/postcodes/{}", postcode);
        let res: PostcodeApiResponse<PostcodeLocation> = reqwest::get(&postcode_url).unwrap().json().unwrap();
        println!("response: {:?}", res);
        res.result
    }
}
