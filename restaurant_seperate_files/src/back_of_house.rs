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