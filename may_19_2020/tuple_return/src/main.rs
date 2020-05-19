fn main() {
    let receive = getdata();  //syntax1 receiving tuple
    println!("{:?}",receive);
    println!("{:?}",getdata()); //syntax2 receiving tuple
    let (temp,today) = getdata();  //syntax3 receiving tuple
    println!("{} Temperatur is {}",today,temp); 
}

fn getdata()->(f32,String){
    let day = "Tuesday".to_string(); //String Type
    let day1 = String::from("Sunday"); //String Type

    let temperature = 41.5;
    (temperature,day)
}