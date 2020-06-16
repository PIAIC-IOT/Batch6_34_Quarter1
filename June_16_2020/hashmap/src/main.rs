use std::collections::HashMap;

fn main() {
    let mut covid = HashMap::new();
    let mut country = String::new();
    let mut cases = String::new();
    println!("Entry Country Name");
    std::io::stdin().read_line(&mut country).expect("Error");
    println!("Entry Number of Covid Cases");
    std::io::stdin().read_line(&mut cases).expect("Error");
    covid.insert(String::from("TEMP"),0);
    // let cases :u32 = match cases.trim().parse() {
    //     Ok(data) => data ,
    //     Err(_) => {println!("Cases should be in Integer Format");
                    //0},

    // };
    match cases.trim().parse() {
        Ok(xyz) => {covid.insert(country.trim().to_string(),xyz);} ,
        Err(_) => {println!("Cases should be in Integer Format")},

    };
    
    println!("covid {:#?}",covid);

    let mut food = HashMap::new();
    food.insert("Peshari Karahi",800);
    //          index            value
    food.insert("Paya",300);
    food.insert("Nehari",400);
    food.entry("Paya").or_insert(500);
    food.entry("Khoya Chanay").or_insert(100);
    println!("Food Hashmap{:#?}",food);
    println!("Paya price : {:?}",food.get("Paya"));
    println!("Chicken Jalfreezi price : {:?}",food.get("Chicken Jalfreezi"));

    let mut num = HashMap::new();
    num.insert(1,"One");
    num.insert(2,"Two");
    num.insert(3,"Three");
    num.insert(30,"Thirty");
    println!("Num Hashmap {:#?}",num);
    println!("index Hashmap {:#?}",num[&2]);
    println!("index Hashmap {:#?}",num.get(&30));
    println!("index Hashmap {:#?}",num.get(&100));

    let mydata = "Dil Dil Pakistan Jan Jan Pakistan Zindabad";
    let mut counting = HashMap::new();
    for temp in mydata.split_whitespace(){
        let xyz = counting.entry(temp).or_insert(0);
        *xyz += 1;
    }

    println!("Word Counting : {:#?}",counting);
}

