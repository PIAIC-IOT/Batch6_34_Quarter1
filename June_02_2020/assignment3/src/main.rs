#[derive(Debug)]
struct Student {
    name : String,
    age : u8,
    grade : String,
    percentage : f32
}

impl Student {
    fn construct(name:String,age:u8,grade:String,percentage:f32)-> Student {
        Student {
            name,
            age, 
            grade ,
            percentage
        }
    }
    fn ppt(&self){
        println!("{}",self.percentage);
    }
}

fn main() {
    let student1 = Student::construct(String::from("Hassan Naseer Malick"),23,String::from("A-1"),84.4);
    println!("{:#?}",student1);
    student1.ppt();
}
