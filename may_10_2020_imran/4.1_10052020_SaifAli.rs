fn main() {
   //Scope # 1 -----------------
   let x: i32 = 5;
 
   {
       //Scope # 2------------------
       let y = 6;
       println!("value of y: {}", y);
   } //Scope 2 ends here
 
   //Here y is no longer valid.
   //println!("value of y: {}", y);
   
   //Here x is still valid.
   println!("value of x: {}", x);
 
  
   /*---------------------------------------------------------------------------------------
   ---------------------------------------------------------------------------------------
   ---------------------------------------------------------------------------------------*/
   let mut primitive_str = "Hello!";
   let mut complex_str = String::from("Hello");
   primitive_str = "World!";
   println!("value of content: {}", primitive_str);
 
   complex_str.push_str(" World!");
   println!("value of content: {}", complex_str);
 
   /*  Ways our variables and data interact: */
 
   //Move with Primitive Data Type: Integer
   let num = 8;
   println!("value of num before interaction: {}", num);
   let num2 = num;
   println!("value of num after interaction: {}", num);
   println!("value of num2 after interaction: {}", num2);
 
   //Move with Complex Data Type: String
   let s = String::from("hello");
   println!("value of s before interaction: {}", s);
   let s1 = s;
   //println!("value of s after interaction: {}", s);
   //println!("value of s1 after interaction: {}", s1);
 
   //Clone
   let s2 = s1.clone();
   println!("value of s1 after interaction: {}", s1);
   println!("value of s2 after interaction: {}", s2);
 
   let to_pass_str = String::from("hello");
   takes_ownership(to_pass_str);
 
   //println!("value of to_pass_str after interaction: {}", to_pass_str);
 
   let to_pass_int = 5;
   makes_copy(to_pass_int);
   println!("value of to_pass_int after interaction: {}", to_pass_int);
   
   
   
    let str_get_ownership = gives_ownership();
    let str_give_ownership = String::from("hello");
    let str_check = takes_and_gives_back(str_give_ownership);
    println!("value of str_check after interaction: {}", str_check);
} //Scope 1 ends here.
 
fn takes_ownership(some_string: String) {
   println!("{}", some_string);
}
 
fn makes_copy(some_integer: i32) {
   println!("{}", some_integer);
}
 
 
 
fn gives_ownership() -> String {
   let some_string = String::from("hello");
   some_string
}
 
fn takes_and_gives_back(a_string: String) -> String {
   a_string
}
 

