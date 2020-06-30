use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut covid_scores = HashMap::new();
    covid_scores.insert(String::from("Pakistan"), 89);
    covid_scores.insert(String::from("Belgium"), 15);
    covid_scores.entry(String::from("Italy")).or_insert(30);
    println!("{:#?}",covid_scores);

    let pk = covid_scores.get("Pakistan");
    println!("{:?}",pk);

}
