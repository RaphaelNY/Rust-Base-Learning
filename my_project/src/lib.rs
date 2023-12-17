mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn sear_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
 // father module can't call child module's function and other things witch is private
 // but child module can call father module's function and other things witch is public
 // the same level module can call each other's function and other things witch is public
pub fn eat_at_restaurant() {
     // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
     // error: crate::front_of_house::hosting::sear_at_table(); // sear_at_table is a private function
     // Relative path
    front_of_house::hosting::add_to_waitlist();
}
////////////////////////////////////////////////////////////////////////////////////////////////////
 // path(2)
fn serve_order() {}

mod back_of_house {
     // use super::serve_order;

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super means father module
        crate::serve_order(); // crate means root module
         // serve_order(); // super means father module
    }

    fn cook_order() {}
}
////////////////////////////////////////////////////////////////////////////////////////////////////
 // path(3) pub struct
mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
     /*
        - struct is public,
        - but, the varies in struct is private if you don't add pub to varies
     */
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
     // error: meal.seasonal_fruit = String::from("blueberries");
}
////////////////////////////////////////////////////////////////////////////////////////////////////
 // path(4) pub enum
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
     // enum is public
     // the varies in enum is public
}
pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}
////////////////////////////////////////////////////////////////////////////////////////////////////