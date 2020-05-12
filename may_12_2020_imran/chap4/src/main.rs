fn main() {
    let age1  :u8 = 22;  //pop
    println!("Age is {:?}",age1);  
    //let food = String::new();
    let food = String::from("Zinger Burger");
    println!("Food is {:?}",food);
    kfc(food);  //ownership of food moved to kfc
    //println!("{}",food); //it will give us error
    let food2 = String::from("Mac Flurry");
    mcdonald(food2.clone());//food2 remain in scope
    println!("after clone {}",food2); //it will print here
    let mut chai = String::new();
    chai.push_str("Mix Tea");
    println!("data of chai {}",chai);
    println!("Pointer/address/Referecen of chai {:p}",&chai);
    println!("Length of chai {}",chai.len());
    pc(&chai);
    println!("After sending to PC");
    println!("data of chai {}",chai);
    println!("Pointer/address/Referecen of chai {:p}",&chai);
    println!("Length of chai {}",chai.len());

    println!("End of Program");
    let mut food3 = String::new();
    println!("Before seding {:?}",food3);
    tandoadam(&mut food3);
    println!("After seding {:?}",food3);

} //drop food2 (pop age1) //drop chai  //food3 drop
fn kfc(deal:String) { //deal takes the ownership
    println!("We are in KFC");
    println!("Today deal is : {}",deal);
}//drop deal
fn mcdonald(deal1:String){ //deal1 String Type
    println!("We are in McDonald");
    println!("Today deal is : {}",deal1);
}//deal1

fn pc(tea:&String){
    println!("We are in Pathan Continental");
    println!("Today deal is : {}",tea);
    println!("Address of Chai is : {:p}",tea);
    println!("Address of Tea is : {:p}",&tea);

}//drop will not call here

fn tandoadam(today:&mut String){
    println!("We are in TandoAdam");
    println!("before push {}",today);
    today.push_str("Mutton Sajji");
    println!("After push {}",today);
}//drop will not call here