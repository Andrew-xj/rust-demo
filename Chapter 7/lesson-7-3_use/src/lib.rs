mod front_of_house;

// absolute
// use crate::front_of_house::hosting; 
// code outside the file cannot use hosting because use is private (by default)
// to enable outside code to use, should use "pub use"
pub use crate::front_of_house::hosting;

// relative
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
