struct Boeing {
    required_crew: u8,
    range: u16,
}

struct Airbus {
    required_crew: u8,
    range: u16,
}

trait Flight {
    fn is_ilegal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_ilegal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 >= distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_ilegal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 280 >= distance) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let boeing = Boeing {
        required_crew: 4,
        range: 7370,
    };

    let airbus = Airbus {
        required_crew: 7,
        range: 5280,
    };

    let boeing_is_legal = boeing.is_ilegal(boeing.required_crew, 18, boeing.range, 2385);
    let airbus_is_legal = airbus.is_ilegal(airbus.required_crew, 4, airbus.range, 2385);

    println!(
        "Boeing is legal: {}\nAirbus is legal: {}",
        boeing_is_legal, airbus_is_legal
    );
}

//methods
//Traits
// Traits are a way to share behavior between different types.
//Language: rust
