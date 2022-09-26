fn main() {
    let x = 2;

    if x < 3 {
        println!("The number {x} is less than 3");
    } else {
        println!("The number {x} is greater than 5");
    }

    // Conditionals must be a boolean; since x is an integer and not a boolean, this throws a
    // compile error
    // if x {
    //     println!(The number {x} is true");
    // }

    if x != 0 {
        println!("The number {x} is not zero");
    }

    // Since if is an expression, we can use it in a let statement
    let x = if false { 5 } else { 0 };
    println!("The value of x is {x}");

    // Types in an expression must match, or else a compile error is generated
    // let x = if false { 5 } else { "zero" };
    // println!("The value of x is {x}");

    // loop loops until told to stop
    let mut i = 5;
    loop {
        if i <= 0 {
            break;
        }

        println!("Loop {i}");

        i -= 1;
    }

    // Loops can have labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");

        let mut i = 10;
        loop {
            println!("Inner loop: {i}");

            if i <= 5 {
                // we can define which loop to break to
                break;
            }

            if count >= 2 {
                break 'counting_up;
            }

            i -= 1;
        }
        count += 1;
    }

    // While loops...
    let mut i = 3;
    while i > 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("GO!");

    // for is useful for looping through collections
    let arr = [10, 20, 30, 40, 50];
    for elem in arr {
        println!("elem: {elem}");
    }

    // Here we create a range of numbers between 1 and 4 (excluding 4) and reverse it, so the first
    // element is the last item in the range and the first element is the last...
    let r = (1..4).rev();
    for num in r {
        println!("{num}...");
    }
    println!("GO");

    // Ranges can be either inclusive or exclusive:
    for num in (1..=3).rev() {
        println!("{num}");
    }
    println!("GO");
}
