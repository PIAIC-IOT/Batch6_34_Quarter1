fn main() {
    ukashahome(String::from("Bubly"));
                            
    let fruit = ["Melon","Water Melon","Chiko","Berries (Falsa)"];
    //            0        1             2      3   
    let price = [60,45,100,300];
    //            0  1  2   3
    let fruit1 = ("Melon","Yellow",1.5,60);
    //   index     0        1       2   3

    println!("Fruit array index 3 {}",fruit[3]);
    println!("comple Fruit array  {:?}",fruit);
    println!("Fruit Tuple index 3 {}",fruit1.1);
    println!("comple Fruit1 tuple  {:#?}",fruit1);  

    for myfruit in fruit.iter(){
        print!("my fruit is {}  ",myfruit);
    }

    let myfruitlenth= fruit.len();
    println!("I have {} fruits",myfruitlenth);

    let salary = 10_000;  //implicit i32
    let age : u8 = 25; //explicity u8
    let number1 : u8 = 66;
    let number2 : f32 = 44.44;
    let plus1 = number1 + number2 as u8;
    //           u8        f32
    println!("u8 {}  + f32 {} = {}",number1,number2,plus1);
    let plus2 = number1 as f32 + number2;
    //           u8         f32
    eid(); // function calling

    println!("u8 {}  + f32 {} = {}",number1,number2,plus2);

    ukashahome(String::from("Belgium Black Chocolate"));
            //  argument
    let eidi = aliraza();
    println!("eidi from Ali Raza {}",eidi);
    println!("my eidi {}",aliraza());
    //hex   0 9  10 11 12 13 14 15 
    //dec   0 9  a   b  c  d  e  f
    let hexdata = 0xa;
    println!("hex number in decimal {}",hexdata);
    println!("hex number   {:x}",hexdata);
    let octdata = 0o16;
    println!("octal number in decimal {}",octdata);
    println!("octal number  {:o}",octdata);
    let bindata = 0b10001;
    println!("Binary number in decimal {}",bindata);
    println!("Binary number  {:b}",bindata);
    let decdata = 100;
    println!("Decimal number in decimal {}",decdata);
    println!("Decimal number in bin {:b}",decdata);
    println!("Decimal number in oct {:o}",decdata);
    println!("Decimal number in hex {:x}",decdata);
    
}

fn eid() //function signature
//next is function definitaion
{  
    println!("Count down starts for Eid Only 19 days left ");
}

fn ukashahome(sweet:String) {
    //       (here we define parameter)
    println!("My sweet from Imran : {}",sweet);
}

fn aliraza() -> u16
//          -> tells compile that this function will return something
{
    println!("I am sending eidi to Ukasha");
    1000 //no semicolon in returning value
}