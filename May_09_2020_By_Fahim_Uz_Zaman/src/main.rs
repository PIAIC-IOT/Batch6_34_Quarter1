// fn main() {
//     println!("Hello, world!");
//     // Array in rust
//     let sameData = [34,54,765,787,34,7];
//     // tuple in rust
//     let diffData = ("name",34,5.8);
//     // combination of array and tuple
//     let hybridData = [("faheem",34,5.8),("naeem",34,5.8),("waseem",34,5.8)];
//     println!("The value is {:?}",hybridData[0]);
//     println!("The value is {}",hybridData[0].1);
// }

// // Function Name which also called function decelaration
// fn largest(arrNum: &[i32])-> i32{
//     let mut max_num =  arrNum[0];
//     for value_check in (1..arrNum.len())
//     {
//         if max_num < arrNum[value_check]{
//             max_num = arrNum[value_check];
//         }
//     }
//     max_num
// }

// fn main() {
//         println!("Hello, world!");
//         // Array in rust
//         let sameData = [34,54,765,1000,34,7];
//         println!("The maximum value is {}",largest(&sameData));
// }


// fn main(){
//     let sameData = [32,53,65,234,76];
//     println!("The Value is {}",sameData[5]);
//     println!("The Length of the array is this: {}",sameData.len());
//     for index in 0..=5{
//         println!("The Values Are: {}",sameData[index]);
//     }
// }

// use std::env;
// fn main(){
//     let value_01 = env::args().nth(1);
//     let value_02 = env::args().nth(2);
//     println!("The values are: {:?} {}",
//         value_01.unwrap(),value_02.unwrap());
// }

// mod sub;
// fn main(){
//     sub::print_hello();
//     let number = 4;
//     // 4=>Function calling is there.
//     println!("The square of the number {} is this {}",number,sub::square(number));
// }



fn main(){
    let sameData = [34,54,6576,86,56];
    for x in sameData.iter(){
        println!("{}",x);
    }
}
