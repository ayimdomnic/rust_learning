mod geo;
use rand::Rng;

//multiproducer singleconsumer - many channels can send messages to the channel only one can receive

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    println!("{}", random_number);
}


