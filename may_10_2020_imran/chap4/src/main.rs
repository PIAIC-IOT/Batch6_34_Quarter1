fn main() {
    let mut course = String::from("IOT");
    println!("content {}",course);
    println!("pointer {:p}",&course);
    println!("Length {}",course.len());
    println!("Capacity {}",course.capacity());
    course.clear();
    println!("After Clear");
    println!("content {}",course);
    println!("pointer {:p}",&course);
    println!("Length {}",course.len());
    println!("Capacity {}",course.capacity());
    course.push_str("IOT Batch34");
    let imran_course = &course;
    println!("after address");
    println!("content {}",imran_course);
    println!("pointer {:p}",imran_course);
    println!("Length {}",imran_course.len());
    println!("Capacity {}",imran_course.capacity());


}
