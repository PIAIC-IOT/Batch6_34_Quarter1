#[derive(Debug)]
struct Food {
   resturant: String,  //field
   food_item:String,
   size:u8,
   price:u16,
   availablilty:bool
} //blue print

fn main() {

   let pizza = Food {
       resturant: String::from("Pizza Hut"),
       food_item:String::from("Chicken Fajita"),
       availablilty:true,
       price:1500,
       size:16,
   };


   let karahi = Food {
       resturant: String::from("BBQ tonight"),
       food_item:String::from("Chicken Ginger"),
       size:1,
       price:1500,
       availablilty:true
   };
   println!("Pizza price {}",pizza.price);
   println!("Pizza {:#?}",pizza);
   println!("Karahi {:#?}",karahi);
   pizza.billing("Habib Ullah".to_string());

   Food::eid_fitr(1_000);
//    printing(pizza);
//    let mytea=pc();
//    println!("My Tea {:#?}",mytea);
//    println!("geting from pc {:#?}",pc());
}

fn printing(data:Food){
   println!("We are in printing function");
   println!("Pizza price {}",data.price);
   println!("Pizza {:#?}",data);
   
}

impl Food {
    fn billing(&self,rider:String){
    println!("We are in billing Method rider id {}",rider);
    println!("Food price {}",self.price);
    println!("Food {:#?}",self);
    }
    fn eid_fitr(eidi:u16){
        for mybalance in 0..eidi {
            println!("My eidi Balance {}",mybalance);
        }
    }
}

fn pc()->Food {
    let chai = Food {
        resturant: String::from("Pathan"),
        food_item:String::from("Mix Tea"),
        size:2,
        price:100,
        availablilty:true
    };
    chai
}

