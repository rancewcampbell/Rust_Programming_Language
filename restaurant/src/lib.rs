use std::fmt;
use std::io as IoResult;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn serve_order() {}

//mod front_of_house {
//    pub mod hosting {
//        pub fn add_to_waitlist() {}
//
//        fn seat_at_table() {}
//    }
//
//    mod serving {
//        fn take_order() {}
//
//        fn serve_order() {}
//
//        fn take_payment() {}
//    }
//}

mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
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
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// absolute path
use crate::front_of_house::hosting;
// relative path
// pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast pleas", meal.toast);

    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
