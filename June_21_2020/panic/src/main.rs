fn main() {
    println!("Hello, world!");
    let mut check = String::new();
    std::io::stdin().read_line(&mut check).expect(" Error");

     let check : u8 = check.trim().parse().expect("We are in trouble in parse");
    //  let check = match check.trim().parse() {
    //     Ok(n) => { println!("Welcom"); n} ,
    //     Err(_) => {panic!("Error after Hello World at line 3"); 0}
    // };
        
    if check >=10 {
        panic!("Panic in in IF");
    }
    println!("After Panic {} ",check);
}
