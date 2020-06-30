fn main() {
    loop {
        let mut age = String::new();
        println!("Enter your Age");
        std::io::stdin().read_line(&mut age).expect("Error");
        let age :u8 = match age.trim().parse() {
            Ok(userage) => userage,
            Err(_)      => continue,
        };
        if age <15 || age >30 {
            println!("Only 15 to 30 years participants are allowed");    
            continue;
        }
        println!("You have entered {}",age);
        break;
    }
}