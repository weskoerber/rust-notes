use crate::garden::vegetables::Asparagus;

// Include code it finds in src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("{:?} is in the garden", plant);
}
