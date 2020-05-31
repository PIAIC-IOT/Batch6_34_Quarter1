fn main () {
    let a:u8 =8;
}//
fn abc() {

}


use std::io;
use rand::Rng;
fn main() {
    let number:u8 = 6;
    if number == 1 {
        println!("You have One");
    } else if number == 2 {
        println!("You have Two");
    } else if number == 3 {
        println!("You have Three");
    } else  {
        println!("You have something else");
    }
    println!("Next line is from Match");
    match number {
        1 => {println!("You have One");},
        2 => {println!("You have Two");},
        3 => {println!("You have Three");},
        4 => {println!("You have Four");},
        5 => {println!("You have Five");},
        _ => println!("Something else")
    }

    
    //                           min  max-1
    
    let lucky = rand::thread_rng().gen_range(1, 50001);
    println!("Your Luck number is : {}",lucky);
    println!("Enter any data");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    println!("32  {:?}",input);
    println!("33  {:?}",input.trim());
    println!("34  {:?}",input);
    let input2 = input.trim();
    println!("36  {:?}",input2);
    input.clear();
    println!("38  {:?}",input);


}
