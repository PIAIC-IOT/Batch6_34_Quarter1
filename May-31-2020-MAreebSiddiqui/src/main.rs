
// use std::io;

// //Referencing 
// fn main () {
//     let name = String::from("Areeb");
//     move_ownership(&name);
//     println!("{}",name);
// }

// fn move_ownership(data: &String) {
//     println!("from the move ownership function: {}",data);
// }

//Illustrating Option Enum

fn increment (num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn main() {
    let seven = Some(7);
    println!("{:?}",seven);
    let result = increment(seven);
    println!("{:?}",result);
    let nothing: Option<i32> = None;
    println!("{:?}",nothing);
}   
