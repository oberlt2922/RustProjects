//module syntax
pub mod front_of_house;

pub mod back_of_house;

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