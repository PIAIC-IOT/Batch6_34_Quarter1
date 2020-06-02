#[derive(Debug)]
enum foode {
    rooti(u8), 
    pizza(String),
    desi(String),
}



impl foode {
    fn ppt(&self){
        println!("{:?}",self);
    }
}

#[derive(Debug)]
struct foods {
    rooti : String, //fields
    pizza : String,
    desi: String,
}
//03343075120 imran ali
fn bill(e1:&foode){
    match e1 {
        foode::rooti(number) => {println!("Your purchased {:?} ",e1);
                            println!("Your Bill is {} Rupees",number*10);},
        foode::pizza(mypiza) => println!("Your purchased {:?} and your bill is Rs.800/-",mypiza),
        foode::desi(mydesi) => println!("Your purchased {:?} and your bill is Rs.800/-",mydesi),
    };
}
fn main() {
    let e1 = foode::rooti(4);
    println!("{:?}",e1);
    e1.ppt();
    bill(&e1);
    let e2 = foode::pizza(String::from("Chicken Fajita"));
    let e3 = foode::desi(String::from("Chicken Mandi"));
    bill(&e2);
    bill(&e3);
    // match e1 {
    //     foode::rooti(number) => {println!("Your purchased {:?} ",e1);
    //                         println!("Your Bill is {} Rupees",number*10);},
    //     foode::pizza(mypiza) => println!("Your purchased {:?} and your bill is Rs.800/-",mypiza),
    //     foode::desi(mydesi) => println!("Your purchased {:?} and your bill is Rs.800/-",mydesi),
    // };

    let s1 = foods {
        rooti : String::from("Garlic Naan"),
        pizza : String::from("Chicken Fajita"),
        desi: String::from("Nali Nehari")
    };
    println!("{:#?}",s1);
    //s1.ppt();

    let number : u8 = 2;
    println!("{}",number);
    let snumber  = Some(88);
    println!("{:?}",snumber);
    let sstring  = Some(String::from("Cornetto"));
    println!("{:?}",sstring);

    match number {
        1 => println!("First Position"),
        2 => println!("Second Position"),
        3 => println!("Third Position"),
        _ => println!("{} Not in Top 3 ",number),
    };

    match snumber {
        Some(88) => println!("you have {:?} rupees ",snumber),
        None => println!("You have nothing"),
        Some(data) => println!("You have {}",data),
    }




}
