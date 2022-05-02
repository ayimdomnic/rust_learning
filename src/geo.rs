const EARTH_RADIUS_IN_KM: f64 = 6371.0;

pub fn distance(start_latitude: f64, start_longitude: f64, end_latitude: f64, end_longitude: f64) -> f64 {
    let start_latitude_radians = start_latitude.to_radians();
    let start_longitude_radians = start_longitude.to_radians();
    let end_latitude_radians = end_latitude.to_radians();
    let end_longitude_radians = end_longitude.to_radians();

    let delta_latitude_radians = end_latitude_radians - start_latitude_radians;
    let delta_longitude_radians = end_longitude_radians - start_longitude_radians;

    let a = (delta_latitude_radians / 2.0).sin().powi(2) +
            (delta_longitude_radians / 2.0).sin().powi(2) *
            start_latitude_radians.cos() *
            end_latitude_radians.cos();

   let distance = 2.0 * EARTH_RADIUS_IN_KM * a.sqrt().asin();

   distance
}