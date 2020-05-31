fn main() {
    let price1 = 300;  //implicit data type i32
    // i32 bit = 4 byte
    // 1 byte = 8 bit
    println!("Today price is  {}",price1);
    println!("addres of price1  {:p}",&price1);  //printing address of a variable
    let price1 = price1 + 100;   //shadowing
    println!("Today price is  {}",price1);
    println!("addres of price1  {:p}",&price1); //printing address of a variable

    let mut salary1:u16 = 10_000;  //10,000
    println!("Salary 1 is  {}",salary1);
    println!("address of Salary1  {:p}",&salary1); //printing address of a variable
    salary1 = salary1+2000;
    println!("Salary 1 is  {}",salary1);
    println!("address of Salary1  {:p}",&salary1); //printing address of a variable    
}