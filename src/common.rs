use std::fmt;

#[derive(Debug)]
pub struct BBox {
    pub lng_min: f64,
    pub lng_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,

    pub lng_radius: f64,
    pub lat_radius: f64,
}

impl BBox {
    pub fn surrounding(point: Point, radius: f64) -> Self {
        let lng_radius = (radius / (111.320 * point.lat.to_radians().cos())) / 2.0;
        let lat_radius = (radius / 110.574) / 2.0;

        BBox {
            lng_min: point.lng - lng_radius,
            lng_max: point.lng + lng_radius,
            lat_min: point.lat - lat_radius,
            lat_max: point.lat + lat_radius,

            lng_radius: lng_radius,
            lat_radius: lat_radius
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
