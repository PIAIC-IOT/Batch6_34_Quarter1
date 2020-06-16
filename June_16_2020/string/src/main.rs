fn main() {
    let  fname = String::from("Mirza");
    let  mname = String::from("Abdur");
    let  lname = String::from("Razzaq");
    //let name = fname + &mname + &lname ;
    let name = format!("{} {} {}",fname ,mname,lname) ;
    println!("Complete name {}",name);
    println!("First name    {}",fname);
    println!("Middle name   {}",mname);
    println!("Last name     {}",lname);
    
    let country = String::from("Pakistan");
    //index                     01234567
    let character = &country[0..1];
    println!("{}",country);
    println!("{}",character);
    let somecharacter = &country[0..=4];
    println!("{}",country);
    println!("{}",somecharacter);

    let utf8 = String::from("こんにちは");
    //index                  012345678901
    println!("lenth of {} is {}",utf8,utf8.len());

    for temp in utf8.chars(){
        println!("{}",temp);
    }

    for temp in utf8.bytes(){
        println!("{}",temp);
    }

}
