fn main() {
    // Create a new vector. Note the type annotation
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // If we have data to start, we can skip the type annotation with a
    // vec macro
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // This won't work, since v isnt mutable
    // v.push(4);
    //
    let mut v = vec![1, 2, 3];
    // Adding elements
    v.push(4);

    println!("{:?}", v);

    // This won't work either; all elements in a vector must be the same
    // type
    // v.push(String::from("Hello, world"));

    let idx = 2;

    // Indexing the vector returns the value, or panics if the index is
    // out of range
    let num = &v[idx];
    println!("Index {} contains the number {}", idx, num);

    let idx = 5;
    // Using the get method returns an option type
    let num = &v.get(idx);
    match num {
        Some(x) => println!("Index {} contains the number {}", idx, x),
        None => println!("Could not retrieve value at index {}", idx),
    };

    // Borrowing rules still work here
    let first = &v[0];
    // v.push(5);
    println!("The first element is {}", first);

    // Iterating the vec with a for loop
    for i in &v {
        println!("{}", i);
    }

    // Changing the vec in the iteration
    for i in &mut v {
        // Dereference the item
        *i *= 50;
        println!("{}", i);

        // We can't do this, because we already borrowed the mutable ref
        // for the for loop; this prevents simultaneous modification
        // v.pop();
    }

    // Holding different types with an enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row: {:?}", row);

    {
        let _v = vec![1, 2, 3];
    } // v goes out of scope and is freed here
}
