use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    println!();

    // method 2 of creating hash maps

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
    println!();

    // hash maps and ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    map.insert(String::from("Favorite movie"), String::from("Batman"));
    // field_name and field_value are invalid at this point

    println!("{:?}", map);
    println!();

    let field_name = String::from("Favorite color");

    // Accessing values in a HashMap
    if let Some(field_value) = map.get(&field_name) {
        println!("{} is {}", field_name, field_value);
    } else {
        println!("No favorite color for me!");
    }

    println!();

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!();

    // overwriting with insert as well
    // Only inserting a value if the key has no value

    map.entry(String::from("Favorite movie"))
        .or_insert(String::from("Superman"));
    map.entry(String::from("Favorite car brand"))
        .or_insert(String::from("Mazda"));

    println!("{:?}", map);
    println!();

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
