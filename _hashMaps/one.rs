
use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    let no_team_name = String::from("Red");
    let no_score = scores.get(&no_team_name).copied().unwrap_or(0);

    println!("{:?}", no_score);
    println!("{:?}", score);

    //iterating over hashMap()

    let mut languages = HashMap::new();

    languages.insert(String::from("Rust"), 1);
    languages.insert(String::from("C++"), 2);
    languages.insert(String::from("Python"), 3);
    languages.insert(String::from("Java"), 4);
    languages.insert(String::from("C"), 5);
    languages.insert(String::from("C#"), 6);
    

    for (keys, languages) in &languages {

        println!("{}: {}", languages, keys);

    }
}