mod front_of_house; // <-- rust will find this file and load it's contents
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}