// parent module
mod front_of_house {
    // child module
    // making a module public does not make children
    // public; they remain private
    pub mod hosting {
        // module paths are private by default
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // modules can contain multiple modules
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    // Declaring a struct public, all fields remain private
    pub struct Breakfast {
        // Each field must be explicitly marked public
        pub toast: String,
        seasonal_fruit: String,
    }

    // Declaring an enum makes all fields public!
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from("toast"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // super refers to the parent module (like .. in a
        // filesystem)
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path (preferred)
    // We can access front_of_house without making it public
    // because the paths are siblings
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Sourdough");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This won't work, as it's private
    // meal.seasonal_fruit = String::From("blueberries");

    // This won't work either; a public associated function is
    // needed to create a struct with private methods
    // let b = back_of_house::Breakfast {
    //     toast: String::from("Rye"),
    // };

    let _app = back_of_house::Appetizer::Soup;
}

// use brings the module into the current scope
use crate::front_of_house::hosting;

// We can do this but it's not idiomatic, and is not preferred
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();

    // Not abundantly clear which module this function belongs to
    add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant2() {
        // This won't work, because use only bring the module
        // into the scope where `use` was used...
        // hosting::add_to_waitlist();
    }
}

// It's idiomatic to use the full path when importing other items
// Rust warns if the imported path is never used
use crate::back_of_house::Breakfast;

// We can alias names brought into scope
use crate::back_of_house::Breakfast as Bekfast;

// We can re-export scopes with pub use. This allows code that
// calls our code to refer to that name as if it had been defined
// in the scope local to the calling code; it essentially short-
// cuts the module path
pub use crate::back_of_house::Breakfast as PubBreakfast;

// Nested paths
// use std::cmp::Ordering;
// use std::path;

// Instead of the above, we can write:
use std::{cmp::Ordering, path};

// This is also valid:
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Glob operator
use std::collections::*;
