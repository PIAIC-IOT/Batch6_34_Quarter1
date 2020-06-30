// use std::fs::File;
 use std::io;
// use std::io::Read;
// use std::{io,io::Read,fs::File};
use std::fs;

fn read_data_from_text() -> Result<String, io::Error> {

    fs::read_to_string("pakistan.txt")
    
    //File::open("pakistan.txt")?.read_to_string(&mut mobeen)?;
    //Ok(mobeen)
}

fn main () {
    println!("{:#?}",read_data_from_text());
}