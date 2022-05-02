use std::ops::{Add, Sub};

#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U,
}

fn main() {
   //declaring a variable as a generic
   //using a constraint
    let vor = NavAid {
         name: String::from("DQN"),
         frequency: 7.5,
         data: String::from("VOR"),
    };

    let ndb_data: Option<String> = Option::None;

    let ndb = NavAid {
        name: String::from("DQN"),
        frequency: 7.5,
        data: ndb_data
    };

    println!("VOR information is: {:?}", vor);
    println!("NDB information is: {:?}", ndb);

    let sum = add(256, 622);

    println!("Sum is: {}", sum);
}

fn add<T: Add<Output = T>>(operand_1: T, operand_2: T) -> T
where T: Add<Output = T> + Sub<Output = T> {
    operand_1 + operand_2
}
