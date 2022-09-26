fn main() {
    {
        // Moving values can be clumsy...
        let s1 = String::from("Hello, world!");

        let (s2, len) = calc_length_move(s1);

        println!("The length of the string '{}' is {}", s2, len);
    }

    {
        // Instead we can pass a reference to the string
        let s1 = String::from("Hello, world!");
        // A reference to s1 is created; this does not move ownership,
        // so s1 is not dropped
        let s1_ref = &s1;
        // Here, the reference to s1 is passed to calc_length_ref
        // without moving ownership of s1 to the function
        // This is called borrowing
        let len = calc_length_ref(s1_ref);

        // Both s1 and s1_ref are valid
        println!("The length of the string '{}' is {}", s1, len);
        println!("The length of the string '{}' is {}", s1_ref, len);

        // References are immutable by default, and cannot be changed
        try_change_borrowed(s1_ref);
    }

    {
        let mut s1 = String::from("Hello");

        {
            // This mutable reference is dropped when the scope ends,
            // meaining the second mutable reference created below will
            // still be valid
            let s1_ref2 = &mut s1;
            s1_ref2.push_str(", world");
        }

        let mut s1_ref = &mut s1;
        // Only one mutable reference may exist at a time; this won't
        // work, because there's already a mutable reference to s1
        // declared above
        // let mut s1_ref2 = &mut s1;

        println!("s1 before being borrowed: {}", s1_ref);

        change_borrowed(&mut s1_ref);

        println!("s1 after being borrowed: {}", s1_ref);

        // There is no limitation to how many immutable references there
        // can be
        let mut s2 = String::from("Hello");
        let _s2_ref = &s2;
        let _s2_ref2 = &s2;

        s2.push_str(" world!");

        // However, we cannot create a mutable reference from an
        // immutable reference: BIG NO-NO
        // This won't work:
        // let _r3 = &mut _s2_ref;
    }

    {
        let s = String::from("Hello");
        let r1 = &s;
        // Mutable references are not allowed when an immutable reference
        // points to the same value
        // let r2 = &mut s;

        // println!("Mutable ref to s: {}", r2);
        println!("Immutable ref to s: {}", r1);
    }

    {
        let mut s = String::from("Hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // The r1 and r2 immutable references end here, since they are
        // not used below

        // Mutable references are allowed only if the last immutable
        // reference is no longer used
        let r3 = &mut s;
        r3.push_str(", world");
        println!("{}", r3);
    }

    {
        // fn dangle() -> &String {
        //     let s = String::from("Hello");
        //
        //     // s goes out of scope at the end of this function, so
        //     // the reference to s will be invalid; this throws a
        //     // compile time error
        //     &s
        // }

        // let ref_to_nothing = dangle();
    }
}

fn calc_length_move(str: String) -> (String, usize) {
    let len = str.len();

    (str, len)
}

fn calc_length_ref(str: &String) -> usize {
    // str is passed by reference, so calc_length_ref doesn't actually
    // have ownership of str
    str.len()
    // Because calc_length_ref doesn't own str, the value str refers to
    // is not dropped
    // This is called borrowing
}

fn try_change_borrowed(_str: &String) {
    // This results in a compile error, since borrowed values cannot
    // be modified
    // str.push_str(" How are you?");
}

fn change_borrowed(str: &mut String) {
    str.push_str(", world!");
}
