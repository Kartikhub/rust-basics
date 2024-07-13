// use std::fmt::Result;
// // Using alias to distinguish types with same name
// use std::io::Result as IOResult;

// fn function1() -> Result {
// }

// fn function2() -> IOResult<()> {

// }

// Using nested paths
use std::io::{self, Write};

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Mango"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // Panic as seasonal fruit is private
    // meal.seasonal_fruit  =String::from("Apple");
    // In the case of enum all the variants are public if enum is public
    let order1 = back_of_house::Appetizer::Soup;
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    // Updated below statement and used the "use" keyword to bring the hosting
    // module into the scope of eat_at_restaurant
    hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
