use rand::prelude::*;
use std::ops::BitXor;
pub mod algorithms;
pub mod lifetimes;
pub use algorithms::*;
pub mod smart_pointers;
pub use lifetimes::*;

fn main() {
    // lifetimes
    lifetimes();
    smart_pointers::smart_pointer();
    algorithms::algorithms();
}
