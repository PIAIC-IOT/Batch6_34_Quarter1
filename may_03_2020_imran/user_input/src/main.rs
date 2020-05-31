use std::io; //including standard input output library
fn main() { // starting scope of main
    //printing static string
    println!("Type you name");
    //declaring blank String Type Variable
    let mut name = String::new();
    //geting input from user in name String variable
    io::stdin().read_line(&mut name).expect("Panic Error");
    //Printing name variable data to screen
    println!("Your name is : {}",name);
} // ending scope of main
