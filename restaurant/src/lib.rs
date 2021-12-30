//module syntax
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}

        pub fn seat_at_table(){}
    }
    
    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    pub fn cook_order(){}

   #[readonly::make]//makes struct readonly meaning it can only be read from outside module
   //readonly structs can be written from inside module
   //readonly was added to the cargo.toml file
   pub struct Breakfast {
        pub toast: String,
        #[readonly]//disignates specific field as readonly instead of entire struct
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            };
        }
    }

    pub enum Appetizer{
        Soup,
        Salad
    }
}

pub fn serve_order(){}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("Okay, that comes with {}", meal.seasonal_fruit);
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
}

//bring path into scope
//use crate::front_of_house::hosting
//call function
//hosting::add_to_waitlist();
//use crate::front_of_house::hosting as h fives hosing an alias
//pub use crate::front_of_house::hosting makes hosting public to external code

//nested paths
//use std::cmp::Ordering;
//use std::io;
//can be rewritten as
//use std::{cmp::Ordering, io};

//another example
//use std::io;
//use std::io::Write;
//use std::io::{self, Write};

//the glob operator brings all items into scope
//use std::collections::*;