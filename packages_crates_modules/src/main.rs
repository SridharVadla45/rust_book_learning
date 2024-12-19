mod flowers;
mod vegetables;
mod front_of_house;

use flowers::test_flowers_modules;
use vegetables::test_vegetables;
mod garden {

    pub mod pests {
        pub fn test_pest() {
            println!("called test pest sub-module ");
        }
    }

    pub fn test() {
        println!("called test function in garden module ")
    }
}

fn main() {
    println!("Hello, world!");
    garden::test();
    test_vegetables();
    test_flowers_modules();
    garden::pests::test_pest();
}
