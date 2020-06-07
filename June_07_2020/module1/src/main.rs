mod finance {
    
    pub fn salary(data:u32){
        println!("Your salary is transfered to your Bank i.e Rs.{}",data);
    }
}

mod hr {
    pub mod imran{
        pub mod shahzad {
            pub fn recruit(){
                println!("There is a new vacancy");
            }
        }
    }
}

mod sales {
    pub fn today_sale(){
        println!("Today sales is Rs.50,000,000");
    }
}

mod marketting{
    pub fn display(){
        println!("We have to display a banner at National Stadium");
    }
}

//use crate::hr::imran::shahzad; //Absolute path
use hr::imran::shahzad; //relative path

fn main() {
    println!("Hello, world!"); //relative path
    finance::salary(30_000); //relative path
    hr::imran::shahzad::recruit();  //relative path
    crate::hr::imran::shahzad::recruit();  //Absolute path
    shahzad::recruit();

    sales::today_sale(); //relative path
    marketting::display();     //relative path
}
