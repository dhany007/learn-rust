use std::collections::HashMap;

fn main() {
    println!("Hash Maps!");

    // creating a new hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values in hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score is {}", score);

    // iterate over key-value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // overwriting a value
    println!("scores = {:?}", scores);
    scores.insert(String::from("Blue"), 50); // before is 10
    println!("scores = {:?}", scores);

    // adding a key and value only if a key isn't present
    scores.entry(String::from("Yellow")).or_insert(100); // no inserted because key yellow is exist
    scores.entry(String::from("Green")).or_insert(200);
    println!("scores = {:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
