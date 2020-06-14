use std::collections::HashMap;
fn main () {
    

    let fruit = vec![String::from("Water Melon"), String::from("Mango")];
    let price = vec![20, 50];

    let mut daily_price: HashMap<_, _> =
        fruit.into_iter().zip(price.into_iter()).collect();

    println!("{:?}",daily_price);

    println!("Mango price {:?}",daily_price.get("Mango"));
    
    let mut cricket = HashMap::new();
    cricket.insert("England",200);
    println!("{:?}",cricket);
    cricket.insert("Pakistan",195);
    println!("{:?}",cricket);

    cricket.insert("Pakistan",180);
    cricket.insert("Bangladesh",250);

    cricket.entry("Pakistan").or_insert(300);
    cricket.entry("New Zealand").or_insert(300);

    for (index, data) in &cricket {
        println!("{}: {}", index, data);
    }



    //let text = "hello world wonderful world";
    let text = "samajh samajh kar samajh ko samjho";

    let mut map = HashMap::new();

    for data in text.split_whitespace() {
        let count = map.entry(data).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}