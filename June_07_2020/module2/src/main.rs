mod food;
mod lunch;
use lunch::serving;
use lunch::serving as ML;

fn main() {
    println!("Hello, world!");
    food::sweets();
    serving::karhi_chawal();
    ML::karhi_chawal();
}
