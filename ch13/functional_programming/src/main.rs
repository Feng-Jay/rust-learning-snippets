use std::thread;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let add_one_v1 = |x:i32| -> i32 { x + 1 };
    // let add_one_v2 = |x| {x + 1}; // the type can be inferred with the usage
    let add_one_v3 = |x| x + 1; // the type can be inferred with the usage
    // let x = add_one_v1(5);
    // println!("x: {}", x);
    let n = add_one_v3(5);
    println!("n: {}", n);

    // this inference only work to get one type, so the closure must always be called with the same type
    let example_closure = |x:&String| x.clone();
    let s = String::from("hello");
    example_closure(&s);
    // let n = example_closure(5);
    println!("s: {}", s);

    let f = |_| (); // sometimes called the "toilet closure"
    let s = String::from("Hello");
    println!("{:?}",f(s));

    let mut list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");


    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After thread: {list:?}");
    let mut s = String::from("Hello");
    let add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s}");
    add_suffix(&mut s);

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    // println!("v1_iter: {:?}", v1_iter); // v1_iter has been moved to the for loop above

    let v2 = vec![String::from("one"), String::from("two"), String::from("three")];
    let mut v2_iter = v2.iter();
    let tmp = v2_iter.next();
    println!("tmp: {:?}", tmp);
    println!("v2: {:?}", v2);

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    println!("a: {:?}", a);
    println!("v: {:?}", v);
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("b: {:?}", b);
    println!("v: {:?}", v);
    println!("{} {}", a[0], b[0]);

}