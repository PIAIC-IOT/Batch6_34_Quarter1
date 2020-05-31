fn main() {
    let age = 33;  //declare and initialize a variable
    println!("welcome in Live Review Class");
    //printing value of a variable
    println!("The data in age variable is : {}",age);
    println!("Hello, world!"); 
    //declare and initialize a tuple
    let data = (137816,"Inayat",1500,80.88); 
    //index      0       1       2    3
    //printing one element value of an index on an tuple
    println!("Element # 1 {}",data.1);
    //printing complete tuple
    println!("Complete Tuple {:#?}",data);
    //printing complete tuple
    println!("Complete Tuple {:?}",data);
    //let data = ();
    //declare and initialize a array
    let age = [22,33,44,55];
    //         0  1  2  3
    //declare and initialize a array
    let names = ["Areeb","Saif","Imran","Zia Khan"];
    //            0       1       2        3
    //printing one element value of an index on an array
    println!("age  1 {}",age[1]);
    //printing one element value of an index on an array
    println!("Name 1 {}",names[1]);
    //printing complete array
    println!("{:?}",age);
    //printing complete array
    println!("{:#?}",names);

    //declare and initialize boolean variable
    let rain = false;
    let summer = true;
    //printing boolean variable
    println!("Is it raining? {}",rain);
    println!("is Today hot? {}",summer);
}
