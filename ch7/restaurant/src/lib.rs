mod front_of_house;

pub fn eat_at_restaurant(){
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();
    // rel path
    self::front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        test::private_function();
        super::deliver_order();
    }

    mod test{
        pub fn private_function() {
            super::super::deliver_order();
        }
    }
    
    fn cook_order() {

    }

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
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub use back_of_house::Breakfast;

mod another {
    use super::back_of_house::Breakfast;
    pub fn eat_at_restaurant3() {
        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        // meal.seasonal_fruit = String::from("blueberries");
        println!("I'd like {} toast please", meal.toast);
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Order1: {:?}, Order2: {:?}", order1, order2);
}
