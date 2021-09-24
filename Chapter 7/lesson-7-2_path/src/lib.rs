mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {

    // absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();

    // pub struct
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");  // error
}

fn _serve_order() {}

mod back_of_house {

    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    fn _cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum _Appetizer {
        Soup,
        Salad,
    }
}
