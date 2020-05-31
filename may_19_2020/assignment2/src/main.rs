fn main() {
    let age : u8 = 22;
    let height : f32 = 5.5;
    let mut course = String::from("IOT");
    println!("{}",age);
    println!("{}",height);
    println!("{}",course);
    let height_square = printing(age, height, &mut course);
    println!("{}",height_square);
    println!("{}",course);
}
fn printing(age:u8,height:f32,course:&mut String)->f32{
    course.push_str(" Batch 6 -34");
    for temp in 0..age {
        print!("{} ",temp);
    }
    height*height
}