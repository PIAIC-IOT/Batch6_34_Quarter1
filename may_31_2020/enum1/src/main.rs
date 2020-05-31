#[derive(Debug)]
enum Move {
//Move is enum name
    Walk, //variants
    Jump, //variants
    Hop, //variants
    Run, //variants    
}

#[derive(Debug)]
enum Sweet {
    Chocolate(String),
    Sweet(String),
    HomeMade(String),
    Open(String,u32),
}

impl Move {
    //method definition
    fn ppt(&self){
        println!("we got  {:?}",self);
    }
}

//function definition
fn daily(myvalue:Move){
    println!("we in daily function data {:?}",myvalue);

    match myvalue {
        Move::Walk =>  println!("Walk is necessary"),
        Move::Jump =>  println!("You are burning more calories"),
        Move::Hop  =>  println!("You are burning good number of calories"),
        Move::Run  =>  println!("You are burning highest numebr of  calories"),
    };
}

fn main() {
    let mymove = Move::Walk;
                //enum::variant
    println!("My move {:?}",mymove);
    let urmove = Move::Run;
                //enum::variant
    println!("My move {:?}",urmove);
    
    mymove.ppt(); //method calling
    daily(urmove); //function calling
    daily(mymove); //function calling

    let mysweet = Sweet::Chocolate(String::from("Rocher"));
    let ursweet = Sweet::Open(String::from("Peanut Chikie"),1);
    println!("My sweet {:?}",mysweet);
    println!("Your sweet {:?}",ursweet);

}
