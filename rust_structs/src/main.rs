// Defining a struct
// Note the use of String intead of &str; we want the struct to own all of its data
struct User {
    username: String,
    email: String,
    is_active: bool,
    sign_in_count: u64,
}

// Tuple-structs are just like structs, except they don't have field names;
// their fields are positional
struct Color(u8, u8, u8);

// Unit-like struct without fields
struct AlwaysEqual;

fn main() {
    // Constructing a struct
    let user1 = User {
        username: String::from("johndoe"),
        email: String::from("your@email.com"),
        is_active: true,
        sign_in_count: 5,
    };

    // Dot notation to modify or retrieve values from a struct
    println!("{}'s email is {}", user1.username, user1.email);

    // Structs are also immutable by default, can't do this because `user1` was not declared
    // mutable. Individual fields cannot be marked immutable; the entire structure must be declared
    // immutable.
    // user1.sign_in_count += 1;

    let mut user2 = build_user(String::from("testdriver"), String::from("test@gmail.com"));
    user2.is_active = false;
    user2.sign_in_count = 1;

    // Instead of this...
    let _user3 = User {
        username: String::from("newuser"),
        email: String::from("new@email.com"),
        is_active: user1.is_active,
        sign_in_count: user1.sign_in_count,
    };

    // We can use struct update syntax
    // This moves user1 here, since the `username` field is a String, it's data exists in the
    // heap, and it doesn't implement the Copy trait; if there were no String fields in the `User`
    // struct, or we redefined all non-Copy-able fields, a stack-only copy would take place,
    // leaving `user1` alone
    let _user4 = User {
        email: String::from("new@email.com"),
        ..user1 // This needs to come last
    };

    // Tuple-struct initialization
    let red = Color(0xFF, 0x00, 0x00);
    println!("Red: {}, {}, {}", red.0, red.1, red.2);

    // Tuple-structs can be destructured; note the constructor syntax
    let Color(r, g, b) = Color(0x4A, 0x89, 0x5F);
    println!("Color: {}, {}, {}", r, g, b);

    // Declaring a unit-like struct
    let _equal = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    // `username` and `email` are initialized using shorthand struct initialization; the parameter
    // name must be the same name for shorthand initialization to work
    User {
        username,
        email,
        is_active: true,
        sign_in_count: 1,
    }
}
