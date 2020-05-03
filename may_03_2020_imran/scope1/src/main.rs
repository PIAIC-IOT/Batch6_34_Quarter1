fn main() {
    let mut age :u8 = 22;
    { //scope 1
        println!("We are in scope 1");
        println!("value of age is {}",age);
        let student_age = 18;
        println!("age of student is {}",student_age);
    }// end of scope 1
    //following line will no execute
    //println!("age of student is {}",student_age);
    age = age + 5;
    { //scope 2
        println!("We are in scope 2 ");
        println!("value of age is {}",age);
        let student2_age = 15;
        println!("age of student is {}",student2_age);
    }// end of scope 2
    //following line will no execute
    //println!("age of student is {}",student2_age);
}
