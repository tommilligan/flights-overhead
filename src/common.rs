use std::fmt;

#[derive(Debug)]
pub struct BBox {
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
pub struct Point {
    pub lng: f64,
    pub lat: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lng: {}, lat: {})", self.lng, self.lat)
    }
}
