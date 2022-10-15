#[derive(Debug)]
pub struct LatitudLongitude {
    pub lat: f32,
    pub long: f32,
}

pub fn get_hawaii_location() -> LatitudLongitude {
    LatitudLongitude {
        lat: 10.008,
        long: 123.0,
    }
}
