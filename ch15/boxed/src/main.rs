use crate::List::{Cons, Nil};
use std::{ops::Deref, rc::Rc, cell::RefCell};
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            List4::Cons(_, item) => Some(item),
            List4::Nil => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:?}", list);
    dbg!(list);

    let x = 5;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointers created.");

    let mut a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = List2::Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a)); // content in rc is shared, but not mutable


    // interior mutability with Rc and RefCell
    // example:
    // let a = 5;
    // let b = &mut a; // this is not allowed, because a is not mutable

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List3::Cons(Rc::clone(&value), Rc::new(List3::Nil)));
    let b = List3::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List3::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("a before = {:?}", a);
    *value.borrow_mut() += 10;
    if let List3::Cons(ref first_value, _) = b {
        *first_value.borrow_mut() += 10;
    }
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a = Rc::new(List4::Cons(5, RefCell::new(Rc::new(List4::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List4::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
}
