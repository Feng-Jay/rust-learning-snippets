use std::result;

fn largest_i32(list: &[i32]) -> & i32{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd> (list: &[T]) -> &T
{
    let mut largest = &list[0];
    
    for item in list{
        if item > largest { // if you want use > for T, you must tell the compiler that T must implement the PartialOrd trait
            largest = item;
        }
    }

    largest
}

struct Point<T>{
    x: T,
    y: T,
}

impl<D> Point<D> {
    fn x(&self) -> &D {
        &self.x
    }
}

impl Point<i32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

struct Point2<T, U>{
    x: T,
    y: U,
}

impl <X, Y> Point2<X, Y> {
    fn mixup<T, U>(self, other: Point2<T, U>) -> Point2<X, U> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = [102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_i32(&number_list);
    let result2 = largest_i32(&number_list2);
    let result3 = largest_char(&char_list);
    println!("The largest number is {}", result);
    println!("The largest number is {}", result2);
    println!("The largest char is {}", result3);
    let integer = Point { x: 5, y: 10  };
    let float = Point { x: 1.1, y: 4.2  };
    // let wont_work = Point { x: 5, y: 4.0  }; // there is only one T, so both x and y must be the same type
    let will_work = Point2 { x: 5, y: 4.3  }; // with two generic types, x and y can be different types
    println!("Integer Point: ({}, {})", integer.x, integer.y);
    println!("Float Point: ({}, {})", float.x, float.y);
    println!("Point2: ({}, {})", will_work.x, will_work.y);
    println!("The x value of integer point is {}", integer.x());
    println!("The distance from origin of integer point is {}", integer.distance_from_origin());

    let p1 = Point2 { x: 5, y: 10.4  };
    let p2 = Point2 { x: "Hello", y: 'c'  };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
