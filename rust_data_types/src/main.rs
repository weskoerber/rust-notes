fn main() {
    // The compiler can infer this datatype
    let x = 42;
    println!("The value of x is {x}");

    // This is equivalent to the line above
    let x: i32 = 42;
    println!("The value of x is {x}");

    // The compiler can't infer the return type of the `parse()` method
    // let guess = "42".parse().expect("Not a number");

    // Type annotations explicitly define the data type when we want to override what the compiler
    // will infer or if we know better
    let guess: i32 = "42".parse().expect("Not a number");
    println!("The value of guess is {guess}");

    let y: u8 = 0xFF;
    println!("The value of y is {y}");

    // Big numbers can contain '_' for better readability
    let big_num = 42_000_001;
    println!("The value of big_num is {big_num}");

    // Numbers can contain a type suffix
    let x = 0xFFu16;
    println!("The value of x is {x}");

    // We can handle wrapping safely with wrapping_*
    let x = 255u8;
    let add: u8 = "8".parse().expect("Not a number");
    let y = u8::wrapping_add(x, add);
    println!("The value of y is {y}");

    // Without wrapping_*, integer overflows cause a panic in debug mode, but will wrap in release
    // mode!
    // Try uncommenting these lines and run `cargo run --release`...
    // let y = add + x;
    // println!("The value of y is {y}");

    // checked_* puts the result into an option type
    let _y = match u8::checked_add(x, add) {
        Some(num) => println!("The value of y is {num}"),
        None => println!("An overflow occurred"),
    };

    // overflowing_* catches overflows as well, but returns a tuple of the value and a bool
    // signaling whether or not an overflow occurred
    let y = u8::overflowing_add(x, add);
    if !y.1 {
        println!("The value of y is {}", y.0);
    } else {
        println!("An overflow occurred");
    }

    // saturating_* limits values at the upper/lower limits of their types
    let y = u8::saturating_add(x, add);
    println!("The value of y is {y}");

    // Floating-point numbers...
    let f = 4.2;
    println!("The value of f is {f}");
    let f: f32 = 4.2;
    println!("The value of f is {f}");
    let f: f64 = 4.2;
    println!("The value of f is {f}");

    // Booleans...
    let _b = false;
    let _b: bool = false;

    // Characters in rust are 4 byes! They hold a unicode scalar value
    let _c = 'W';
    let _c: char = 'K';
    let _c = '😻';

    // Tuples
    let tup = (u8::MAX, 4.2, false);
    let _tup: (u8, f32, bool) = (u8::MAX, 4.2, false);

    // Tuples can be indexed
    println!("Tuple: {} {} {}", tup.0, tup.1, tup.2);

    // Tuples can be destructured
    let (x, y, z) = tup;
    println!("Tuple: {x} {y} {z}");

    // An empty tuple is called a unit. A unit represents an empty value or empty return type
    let _unit = ();

    // Arrays
    let _arr = [1, 2, 3, 4, 5];

    // Array type define the type of the elements and size of the array
    let _arr: [u8; 5] = [1, 2, 3, 4, 5];

    // This declares an array with 5 elements, with each element being set to 3
    let _arr = [3; 5];
    let arr: [u8; 5] = [3; 5];

    // Array elements may be indexed
    println!("The first element in arr is {}", arr[0]);

    // Out-of-bounds indeces will result in a runtime error, and this code will panic
    let i: usize = "6".parse().expect("Not a number");
    let elem = arr[i];

    // This line will not run, since the program panics at the above line
    println!("The element at index {i} is {elem}");
}
