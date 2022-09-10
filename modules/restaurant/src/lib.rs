mod front_of_house;

fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;
use std::io::Result as IoResult;
use std::{collections::HashMap, fmt::Result};

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}
