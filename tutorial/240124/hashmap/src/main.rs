use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    scores.entry(String::from("Blue")).or_insert(50);      // X
    scores.entry(String::from("Yellow")).or_insert(50);    // O

    let score = scores.get("Blue").copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
