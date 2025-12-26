use std::fmt;
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }    
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


mod inner {
    pub trait A {
        fn f(&self) -> usize { 0 }
    }
    pub trait B {
        fn f(&self) -> usize { 1 }
    }
    pub struct P;
    impl A for P {}
    impl B for P {}
}


fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("Resulting Point: ({}, {})", p3.x, p3.y);
    p3.outline_print();
    let person = Human;
    person.fly(); // Calls Human's own fly method
    <Human as Pilot>::fly(&person); // Calls Pilot's fly method
    <Human as Wizard>::fly(&person); // Calls Wizard's fly method

    // give the self can fetch the implementation of the specific trait
    // however, associated functions don't have self parameter
    // use following fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    use inner::{P, B};    
    println!("{}", P.f());
}
