fn main() {
    let hellourdu = String::from("السلام عليكم");
    println!("Hello in urdu {}",hellourdu);
    let hellochina = String::from("你好");
    println!("Hello in Chinese {}",hellochina);

    let pakchina = hellourdu + &hellochina;

    println!("Urdu + Chinese {}",pakchina);
    
    let name1 = "Abdullah ".to_string();
    let age = 24.to_string();
    let attock = format!("{} {}",name1,age);
    println!("{}",attock);
    println!("{}",name1);
    println!("{}",age);

    let fruit = String::from("Water Melon");
    //index                   0123456789
    let partial_fruit = &fruit[6..=10];
    println!("whole fruit {}",fruit);
    println!("partial_fruit  {}",partial_fruit);

    for temp in fruit.chars(){
        println!("{}",temp);
    }
    for temp in fruit.bytes(){
        println!("{}",temp);
    }

}
