use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {
        id: 1,
        name: String::from("Fresh Asparagus"),
    };
    println!("{:?}", plant);
    garden::plant_seed();
    let test = garden::TestStruct::new(1, String::from("Example"));
    println!("testStruct a: {}, b: {}", test.a, test.b);
}
