mod geo;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    println!("{}", random_number);
    geo::distance(0.0, 0.0, 0.0, 0.0);
}


