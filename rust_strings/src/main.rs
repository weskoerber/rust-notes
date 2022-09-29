fn main() {
    // String is a wrapper around a Vec<T>
    let mut s = String::new();
    s.push('c');

    println!("{}", s);

    // to_string is available for any type that implements
    // the Display trait
    let data = "data.to_string()";
    let s = data.to_string();

    println!("{}", s);

    // to_string works on a literal
    let s = "literal to_string()".to_string();
    println!("{}", s);

    // Another method...
    let s = String::from("String::from()");
    println!("{}", s);

    // Strings are UTF-8
    let hello = "こんにちは";
    println!("{}", hello);

    let mut s = String::new();
    s.push('こ');
    println!("{}", s);

    // Strings can be combined
    let mut s = "hello".to_string();
    s.push_str(" world");
    println!("{}", s);

    // Concatenation with + or format!
    let s1 = "Hello, ".to_string();
    let s2 = "world!";
    // s1 is moved and can't be used after this, but s2
    // was borrowed and can be used
    // `+` uses the add method under the hood which takes
    // a &str, but compiler uses deref coersion to coerce
    // the &String to a &str[..]
    let s3 = s1 + &s2;
    println!("{}", s3);

    // string formatting
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", s);

    // Not allowed to index a String
    let _s = String::from("hello");
    // This is not allowed
    // let c = s[0];

    // The best way to operate on strings is to be explicit
    // about whether you want characters or bytes
    for c in "こんにちは".chars() {
        println!("{}", c);
    }

    for b in "こんにちは".bytes() {
        println!("{}", b);
    }
}
