mod irfan {
    #[derive(Debug)]
    pub enum Fruit {
        WaterMelon,
        Cherries(String),
        Jamun,
        Beeris_Falsay
    }
    #[derive(Debug)]
    pub struct Guest {
        pub name:String,
        pub age:u8
    }

}
fn main() {
    println!("Hello, world!");
    let myfruit = irfan::Fruit::Jamun;
    println!("My fruit {:?}",myfruit);
    let guest2 =  irfan::Guest {
        name: String::from("Irfan Haider"),
        age : 51
    };
    println!("Guest 2 {:?}",guest2);
}
