use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let gui = None;
    let hit: Option<i32> = 89;
    let git: Option<i8> = None;
    let str = "hijab";
    
}
