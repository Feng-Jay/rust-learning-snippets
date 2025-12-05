use restaurant::eat_at_restaurant2;
use restaurant::Breakfast;
use rand::Rng;
// use std::collections::HashMap;
use std::collections::*;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};


fn main() {
    eat_at_restaurant2();
    let mut meal = Breakfast::summer("Rye");
    let seceret_number = rand::thread_rng().gen_range(1..101);
    println!("Hello, world! The secret number is: {}", seceret_number);
}