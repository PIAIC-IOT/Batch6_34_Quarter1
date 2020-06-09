mod lunch;
mod dinner;
//use lunch::food;
//use dinner::food::dall;
use dinner::food::{self,dall};

use dinner::food::chicken::achargosht;

fn main() {
    println!("Hello, world!");
    lunch::food::pasta::macroni();
   // food::rice::biryani();
    achargosht();
    nashta();
    dall::dallfry();
    food::chicken::achargosht();

}
fn nashta(){
    println!("nashta");
}
