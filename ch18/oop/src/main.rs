use oop::{Draw, Screen, Button};
use std::fmt::Debug;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box: ({}x{}) with options: {:?}", self.width, self.height, self.options);
    }
}

pub fn main(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    let n = 1;
    let s = String::from("Hello");
    let v: Vec<&dyn Debug> = vec![&n, &s];
    // let n_ref = v[0] as &i32; // an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    // println!("{}", n_ref + 1);
}