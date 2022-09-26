#[derive(Debug)]
struct Rect {
    height: f32,
    width: f32,
}

fn main() {
    let r1 = Rect {
        height: 12.5,
        width: 13.0,
    };

    let a1 = area(&r1);
    println!(
        "Rectangle width ({}) x height ({}) = {}",
        r1.width, r1.height, a1
    );

    // This takes ownership of the value and prints to stderr
    // Note the borrow; r1 would be invalid below if it was moved here!
    dbg!(&r1);

    // dbg!() returns ownership, so this is valid (r1 is shadowed)
    let r1 = dbg!(r1);

    let width_fact = 0.2;
    let r2 = Rect {
        height: 0.55,
        width: dbg!(1.0 * width_fact),
    };

    dbg!(&r2);

    // This won't work because it doesn't implement the std::fmt::Display trait:
    // println!("r1: {}", r1);

    // {:?} says we should use the output format called Debug
    // The struct must derive the Debug trait for this to work
    println!("r1: {:?}", r1);

    // This pretty-prints the struct
    println!("r1: {:#?}", r1);
}

fn area(r: &Rect) -> f32 {
    r.width * r.height
}
