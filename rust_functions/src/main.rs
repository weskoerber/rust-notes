fn first_function() {
    println!("first_function");
}

fn main() {
    println!("Hello, world!");

    first_function();
    another_function();

    print_num(42);

    multi_param(5, true);

    // This is a statement; it performs some action and does not return a value
    let _x = 5;

    // To prove this, this doesn't work:
    // let _x = (let x = 5);

    // Expressions evaluate to a value
    let _x = 5 + 6;

    // New scope blocks are expressions as well
    let y = {
        let x = _x + 8;

        // Expressions do not end in semicolons
        x + 1

        // Semicolons turn expressions into statements - statement do not return a value; the line
        // below causes compile errors
        // x + 1;
    };
    println!("The value of y is {y}");

    println!("Five: {}", five());
    println!("5 + 1 = {}", increment(five()));
}

// Function definitions can be located anywhere in code
fn another_function() {
    println!("another_function");
}

// Function with a parameter
fn print_num(num: i32) {
    println!("print_num: {num}");
}

fn multi_param(num: i32, b: bool) {
    println!("multi_param: {num}, {b}");
}

fn five() -> i32 {
    // This is an expression; therefore, it returns a value (in this case, 5)
    5
    // The line above is equivalent to:
    // return 5;
}

fn increment(x: i32) -> i32 {
    x + 1
}
