mod front_of_house; // Import from file named front_of_house.rs

pub use crate::front_of_house::hosting; // Bring hosting into scope crate::

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}