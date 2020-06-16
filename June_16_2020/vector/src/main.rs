use std::io;
fn main() {
    let mut age: Vec<u8> = Vec::new();
    loop {
        println!("Enter Age");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        // match input.trim().parse() {
        //     Ok(temp) => {if temp==0 {
        //                     break;
        //                     } else {
        //                         age.push(temp);
        //                     }
        //                 },
        //     Err(_) => {println!("Only Numbers allowed");
        //                 },
        // }
           match input.trim().parse() {
            Ok(temp) => { 
                        match temp {
                            0 => break,
                            _ => age.push(temp)
                        }
                         },
            Err(_) => {println!("Only Numbers allowed");
                        },
        }
    }
    println!("{:?}",age);

    let data = match age.get(2) {
        Some(xyz) => xyz,
        None       => &0,
    };
    println!("{:?}",data);
    println!("{:?}",data*2 );

}
