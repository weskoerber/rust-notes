#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // ...
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    // The match expression compares a values against a series of patterns
    // and executes code based on the matching pattern
    //
    // match arms can have curly braces
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // match arms can bind the parts of the values that match the
        // pattern...
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// Handling option types with match
fn plus_one(x: Option<i32>) -> Option<i32> {
    // match arms must be exhaustive; try commenting out one of the arms
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("A Dime is {} cents", value_in_cents(Coin::Dime));
    println!("A Penny is {} cents", value_in_cents(Coin::Penny));
    println!(
        "A Quarter is {}",
        value_in_cents(Coin::Quarter(State::Alaska))
    );

    let _five = Some(5);
    let _size = plus_one(_five);

    let age = 20;
    match age {
        18 => println!("You can buy a rifle"),
        21 => println!("You can buy alcohol"),
        25 => println!("You can rent a car"),
        // This passes all other values to this arm (e.g 17, 24, 76, ...)
        other => println!("You are {}", other),
        // Rust warns of us an unreachable pattern here, since a
        // catch-all was defined above
        55 => println!("You are old"),
    };

    match age {
        40 => println!("Over the hill!"),
        // This is a catchall that doesn't pass the value to the arm
        _ => println!("Happy birthday #{}!", age),
    };

    match age {
        100 => println!("You're REALLY old"),
        // This returns the unit type and doesn't really do anything
        _ => (),
    };
}
