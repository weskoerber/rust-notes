#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // ...
    Wyoming,
}

enum Coin {
    Penny,
    Nickey,
    Dime,
    Quarter(State),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => (),
    }

    // If let is more consise than the above match expression.
    // This is syntactic sugar for a match expression having a single
    // matching pattern and ignoring everything else
    if let Some(max) = config_max {
        println!("The max is {}", max);
    };

    // if let can have else
    let mut count = 0;
    let coin = Coin::Quarter(State::Wyoming);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
