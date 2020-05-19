
fn main() {
    //integer to string
    let age : u8 = 66;
    println!("{:?}",age);
    let string_number = 44.to_string();
    //let string_number = "MIRZA".to_string();
    println!("{:?}",string_number);
    let string2number:u8 = string_number.parse().unwrap();
    println!("{:?}",string2number);
    println!("Type your age");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    println!("You entered {:?}",input);
    println!("You entered {:?}",input.trim());
    //let input_intger :u8 = input.trim().parse().unwrap();
    let input_intger :u8 = input.trim().parse().expect("Wrong data type Only Interger are valid for age");
    println!("Your converted integer {:?}",input_intger);

}
