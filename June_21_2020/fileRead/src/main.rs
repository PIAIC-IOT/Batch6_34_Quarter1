use std::fs::File;

// use std::io;
// use std::io::Read;

use std::io::{self,Read};

fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    
    // let f = File::open("hello.txt");

    // let mut path = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut data = String::new();

    // match path.read_to_string(&mut data) {
    //     Ok(_) => Ok(data),
    //     Err(e) => Err(e),
    // }
}


fn main () {
    let ourdata = read_from_file();
    println!("{:#?}",ourdata);
}