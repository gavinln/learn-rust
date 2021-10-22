use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores is {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores2 is {:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map is {:?}", map);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score is {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("scores is {:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("scores is {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(17);
    println!("scores is {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
