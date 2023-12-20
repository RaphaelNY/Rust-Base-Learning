mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function() {}
    }
}

 // use crate::front_of_house::hosting; // this path is private
 // use self::front_of_house::hosting;
 // use front_of_house::hosting;
pub use crate::front_of_house::hosting; // this path is public.
 // pub use self::front_of_house::hosting;
 // pub use front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
     // error: hosting::some_function();
}

 // use: function and same_name's others always use to his nearest father module
    // but struct, enum, and other items use to his top father module