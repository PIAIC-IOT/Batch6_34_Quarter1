use std::io;

fn main() {
    let mut age : Vec<u8> = Vec::new();
    println!("lenght {:?}",age.len());
    age.push(26);
    println!("after first element {:?}",age);
    age.push(24);
    println!("after second element {:?}",age);
    println!("lenght {:?}",age.len());
    age.pop();
    println!("after pop element {:?}",age);
    println!("lenght {:?}",age.len());

    // let mut salaryK = [20,30,50];
    // //                 0   1  2
    // println!("array {:?}",salaryK);
    // salaryK[2]=60;
    // println!("array {:?}",salaryK);
    // salaryK.push(70);
    // println!("{:?}",salaryK);
    let mut fruit = vec![String::from("Mango")];
    println!("fruit vector {:?}",fruit);
    let mut temp = String::new();
    println!("What is your favourite fruit");
    io::stdin().read_line(&mut temp).expect("You are in trouble");
    fruit.push(temp.trim().to_string());
    println!("fruit vector after user input {:?}",fruit);

    println!("Second Index {}",fruit[1]);
    println!("First Index {:?}",fruit.get(0));

    for data in fruit{
        println!("we are in for loop {}",data);
    }

    let mut v = vec![100, 32, 57];
    println!("before change {:?}",v);
    for i in &mut v {
        *i += 50;
    }
    println!("after  change {:?}",v);



    
}
