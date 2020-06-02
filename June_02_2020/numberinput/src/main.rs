// Internet connection would be required to excute this program first time

// add follwoing in cargo.toml under [dependencies]
// text_io = "0.1.7"

#[macro_use]
extern crate text_io;   //using external codes
fn main() {  //main function start

    println!(" enter data ");       //print static string

    let  input: i32 = read!();         //declare variable and get input

    println!(" enter data {}",input);         //print your entered data

} //scope ends of main function