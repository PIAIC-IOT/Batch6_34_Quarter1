
fn main() {
    let mut food = String::from("Biryani ");
    println!("Food : {}",food);
    add_food(&mut food);
    println!("Food : {}",food);
    food.clear();
    add_food(&mut food);
    println!("Food : {}",food);

    let mut mydata = ("".to_string(),0,"".to_string());
    //index           0             1  2   
    let input = "Biryani".to_string();
    mydata.0=input;

}
fn add_food(student:&mut String){
    student.push_str("Raita + Kheer + Mirinda");
}
