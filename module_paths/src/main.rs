use back_of_house::Appetizer;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waiting list ");
        }
    }
}

fn deliver_order() {
    println!("order delivered to the customer ");
}

mod back_of_house {

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        //unassocited to instance but builds the instance
        pub fn order(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook();
        crate::deliver_order();
        super::deliver_order();
        //   deliver_order();// cannot find the path because these are sibilings
    }

    pub fn cook() {
        println!("cooking the ordered food ");
    }


    pub enum Appetizer {
        Soup,
        Salad(String),
    }
}

pub fn eat() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path

    front_of_house::hosting::add_to_waitlist();

    let mut breakfast = back_of_house::Breakfast::order("wheat");

    breakfast.toast = String::from("rye");
   
    println!(" {breakfast:?}");


    let desert_order1= back_of_house::Appetizer::Salad(String::from("gulab Jamun"));

    match desert_order1 {
        Appetizer::Salad(t)=> {
            println!("Desset ordered :  {t}");
        }

        Appetizer::Soup => {
            println!("Ordered soup ");
        }
    }


    
}

fn main() {
    println!("Hello, world!");
    eat();
}
