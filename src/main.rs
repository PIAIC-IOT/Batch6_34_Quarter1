use std::io;

//Referencing 
fn main () {
    let name = String::from("Areeb");
    move_ownership(&name);
    println!("{}",name);
}

fn move_ownership(data: &String) {
    println!("from the move ownership function: {}",data);
}