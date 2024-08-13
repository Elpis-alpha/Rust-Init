mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::back_of_house::{Appetizer, Breakfast};
use crate::front_of_house::hosting;

mod customer {
    use crate::back_of_house::Breakfast;

    pub fn eat_at_restaurant() {
        use std::{collections::HashMap, fmt, io};
        // let mut map = HashMap::new();
        // map.insert(1, 2);

        Breakfast::summer("cats");
    }
}

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    // let mut meal = back_of_house::Breakfast {
    //     toast: String::from("Cats"),
    //     seasonal_fruit: String::from("invalid")
    // };
    meal.toast = String::from("Wheat");

    println!("I'd like a {} toast pklease", meal.toast);

    hosting::add_to_waitlist();

    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup;

    // meal.sea
}

fn deliver_order() {}

pub fn eat_at_restaurantx() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
