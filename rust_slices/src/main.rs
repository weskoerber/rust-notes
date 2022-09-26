fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(" world");

        // indexing slices...
        let hello = &s[0..5];
        let world = &s[6..11];

        // This is equvalent
        // let hello = &s[..5];
        // let world = &s[6..];

        // Out-of-bounds errors are runtime exceptions
        // let error = &s[12..];

        println!("{} {}", hello, world);

        // You can take a slice of the entire string
        let hello_world = &s[..];

        println!("{}", hello_world);

        let first_word = first_word(&s);

        // This won't work, because we borrowed `s` immutably in the line above
        // s.clear();

        println!("The first word in the string '{}' is {}", s, first_word);

        // We can clear the string here since we don't use any more immutable references in this scope
        // s.clear();

        // We can use a function that takes a &str instead of a String, and it's compatible with
        // both
        let slice = &s[..5];
        // This won't work, because &str is not compatible with String, but String is compatible
        // with &str:
        // let a = first_word(slice);
        let first_word_slice = first_word_slice(slice);
        println!(
            "The first word in the string '{}' is {}",
            slice, first_word_slice
        );
    }

    {
        // `s` is a reference to a specific point in the binary; that is why it is immutable
        let s = "I'm immutable";

        println!("'{}' is an immutable string slice (&str)", s);
    }

    {
        // We can also take slices of an array
        let arr = [1, 2, 3, 4, 5];
        let first_3 = &arr[..3];
        for item in first_3.iter().rev() {
            println!("{}...", item);
        }
        println!("LIFTOFF");
    }
}

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    return &str;
}

fn first_word_slice(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    return &str;
}
