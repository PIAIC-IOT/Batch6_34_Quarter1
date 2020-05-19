#[derive(Debug)]
struct User {
    name: String,
    roll: u32,
}
impl User {
    fn printing(&self){ //method
        println!("Welcome in printing method");
        println!("Roll Number of {} is {}",self.name,self.roll);
        println!("Complete user 1 {:#?} ",self);
    }
    fn with_parameter(&mut self,name:String,age:u8){ //method
        println!("Welcome in with_parameter method");
        println!("Age of {} is {} ",name,age);
        self.name.push_str(" Khan");
        println!("Roll Number of {} is {}",self.name,self.roll);
        println!("Complete user 1 {:#?} ",self);
    }
    fn welcome(name:String,roll:u32)->User { //associated function
        println!("Welcome in WELCOME associated function");
        User {
            name: name,
            roll: roll,
        }
        //you can do anything
    }    
}
fn ppt(data:&User){  //simple function
    println!("Welcome in ppt function");
    //you can do anything
}
fn main() {
    let mut user1 = User {
        name: "Nauman Ali".to_string(),
        roll: 96542,
    };
    println!("Roll Number of {} is {}",user1.name,user1.roll);
    println!("Complete user 1 {:#?} ",user1);
    ppt(&user1);  //function calling
    user1.printing(); //method calling
    //User::welcome(); //associated function calling
    let user2= User::welcome(String::from("Adnan Nawaz"),126890); //associated function calling
    user2.printing();
    //stdin().read_line().unwrap();
    User::welcome(String::from("Ukasha"),700007).printing(); //associated function calling
    //println!("{}",danger());
    user1.with_parameter("Shankar Das".to_string(),21);
}

// fn danger()->&String{ //start
//     let ali = String::from("ALi Baloch"); //ali variable heap declate
//     &ali //ali reference
// } //drop ali