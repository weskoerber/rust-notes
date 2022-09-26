fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // This won't work; variables are immutable by default
    // x = 5;
    // println!("The value of x is {x}");

    // It works if we declare the variable as immutable
    let mut y = 6;
    println!("The value of y is {y}");
    y = 5;
    println!("The value of y is {y}");

    // We can create arbitrary scopes
    {
        // This is called shadowing; it redefines the previous value until the scope ends or itself
        // is shadowed
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");

        // We can shadow a variable to a different type
        let y = "Hello, world!";
        println!("The value of the shadowed y is {y}");

        // We can't assign different types to a mutable variable - that only works with shadowing
        // let mut z = 5;
        // z = "Hello, world";
    }

    println!("The value of y is {y}");

    // Constants are _always_ immutable, and must include type annotations
    const SECONDS_IN_24_HOURS: u32 = 24 * 60 * 60;
    println!("There are {SECONDS_IN_24_HOURS} seconds in 24 hours");
}
