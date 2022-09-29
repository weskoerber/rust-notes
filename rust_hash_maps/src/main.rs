use std::collections::HashMap;

fn main() {
    // Hash maps sore a mapping of keys of type K to values
    // of type V (aka dictionary, assoc. array, etc)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get returns an option that contains the reference to the
    // value or None, so the value needs to be copied
    let team_name = String::from("Blue");
    let score = match scores.get(&team_name).copied() {
        Some(score) => score,
        None => 0,
    };
    println!("{} team score: {}", team_name, score);

    // This is the same as the match above
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} team score: {}", team_name, score);

    // iterating (key/value extracted as tuple)
    for (k, v) in &scores {
        // iterating over a hash map happens in an arbitrary
        // order
        println!("{}: {}", k, v);
    }

    let name = String::from("Red");
    let score = 6;

    // Types implementing the Copy trait are copied into the
    // hash map; types that don't are moved
    scores.insert(name, score);

    // name is invalid here!
    // println!("{} added to scores", name);
    // but score was copied, so it's still valid
    println!("Score of {} added", score);

    // Updating a hash map
    // overwriting
    scores.insert(String::from("Blue"), 69);
    println!("{:?}", scores);

    // inserting only if non-existent
    // or_insert returns a mutable reference
    scores.entry(String::from("Yellow")).or_insert(420);
    println!("{:?}", scores);

    // updating existing values
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
