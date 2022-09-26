fn main() {
    // Main ownership rules:
    //   1. Each value has an owner
    //   2. There can only be one owner at a time
    //   3. When the owner goes out of scope, the value is dropped

    // s isn't defined here
    {
        // This is a string literal; it's allocated on the stack and
        // is immutable
        let _s = "hello";
        // s is valid now until the scope ends
        // do stuff with s
    }
    // the scope ends; s is no longer valid

    {
        // This creates a string on the heap
        let mut s = String::from("Hello");
        println!("{}", s);

        // This string can be mutated
        s.push_str(", world!");
        println!("{}", s);
    }
    // The string is now deallocated, as it is now out of scope
    // Rust automatically calls `drop` at the closing curly bracket

    {
        // This creates 2 values on the stack, both equal to 5; first,
        // x is assigned the value 5, then the value of x is copied to y.
        // Scalar data types or data types that do not require allocation
        // may be copied
        let x = 5;
        let y = x;
        println!("The value of x is {x}");
        println!("The value of y is {y}");
    }

    {
        // The String is made up of 3 parts (all of which are allocated
        // on the stack):
        //   - A pointer to the data on the heap
        //   - A length
        //   - A capacity

        // This creates string s1, then s1 (the String "structure" - pointer,
        // length, and capacity - is copied to s2; the data on the heap that
        // the pointer points to is _not_ copied.
        let s1 = String::from("Hello");
        // s1 is moved (shallow-copied) to s1, rendering s1 invalid
        let s2 = s1;
        // This won't work, because s1 was is not longer valid
        // println!("The values of s1 is {}", s1);
        println!("The values of s2 is {}", s2);
    }

    {
        // `clone` deep-copies the value; it copies the structure on the
        // stack as well as the data on the heap to the new value
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }

    {
        // Tuples can be copied only if the all types they contain can
        // be copied
        let t1 = (5, 4.2, false);
        let t2 = t1;
        println!("t2: {}, {}, {}", t1.0, t2.1, t2.2);

        // This won't work, because String doesn't implement the Copy
        // trait, since it's data is allocated on the heap
        let t1: (i32, f32, String) = (5, 4.2, String::from("false"));
        let _t2 = t1;
        // println!("t2: {}, {}, {}", t1.0, t1.1, t1.2);
    }

    {
        // str comes into scope; the data is allocated on the heap
        let str = String::from("Wes");

        // str's value moves into takes_ownership
        takes_ownership(str);
        // str is no longer valid in this scope
        // This won't work:
        // println!("str is no longer valid", x);

        // x comes into scope; it's allocated on the stack
        let x = 5;

        makes_copy(x);
        // x is copied into makes_copy, so it's still valid
        println!("x with value {} is still valid", x);
    }

    {
        // gives_ownership moves its return value into s1
        let s1 = gives_ownership();
        println!("s1: {}", s1);

        // s2 comes into scope
        let s2 = String::from("World");
        println!("s2: {}", s2);

        // s2 is moved into takes_and_gives_back (and therefore is
        // invalid)
        // takes_and_gives_back moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
    // some_string goes out of scope and the memory is freed
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("Hello");

    // gives_ownership will move ownership of some_string into the
    // function that calls it
    some_string
}

fn takes_and_gives_back(str: String) -> String {
    // str comes into scope
    str
    // str is returned and moves out to the caller
}
