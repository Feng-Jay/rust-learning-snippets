pub mod vegetables;

pub fn plant_seed() {
    println!("Planting a seed!");
}

pub struct TestStruct {
    pub a: i32,
    pub b: String,
}

impl TestStruct {
    pub fn new(a: i32, b: String) -> TestStruct {
        TestStruct { a, b }
    }
}