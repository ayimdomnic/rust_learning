use std::collections::HashMap;
//Arrays and tuples
//Stored in stack memory
//collection have multiple types
//collections are stored on the heap,
//so they are allocated on the heap
//can grow and shrink
//sequences, maps & sets
//sequesnce is a collection of items you can add, remove, and iterate over
//ordered lists

fn main() {
    let mut flights = HashMap::new();

    flights.insert("DA918", ("11:45", "Orlando"));
    flights.insert("DA919", ("12:05", "Salt lake city"));
    flights.insert("DA920", ("12:25", "Denver"));
    flights.insert("DA921", ("12:45", "Phoenix"));
    flights.insert("DA922", ("13:05", "Los angeles"));
    flights.insert("DA923", ("13:25", "San diego"));

    let flight_number = "DA918";

    let option = flights.get(flight_number);

    let (time, destination) = option.unwrap();

    println!(
        "The flight {} is leaving at {} and will be going to {}",
        flight_number, time, destination
    );

    if !flights.contains_key(flight_number) {
        println!("Flight {} is not in the database", flight_number);
        flights.insert(flight_number, ("13:45", "New york"));
    } else {
        println!("Flight {} is in the database", flight_number);
    }
}
