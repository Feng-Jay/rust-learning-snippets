#[derive(Debug)]
enum Message {
    Write(String),
    Move(i32, i32),
}

impl Message {
    fn call(&self){
        match self {
            Message::Write(text) => println!("Writing: {}", text),
            Message::Move(x, y) => println!("Moving to coordinates: ({}, {})", x, y),
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime(i32),
    Quarter,
}

fn main() {
    println!("Hello, world!");
    let w = Message::Write(String::from("Hello, Rust!"));
    let m = Message::Move(2, 2);
    w.call();
    println!("Message: {:?}", w);
    let some_number = Some(5);
    println!("Some number: {:?}", some_number);
    !dbg!(some_number.is_none());
    println!("{}",some_number.unwrap_or(0));
    let abs: Option<i32> = None;
    println!("Absolute value: {:?}", abs.unwrap_or(0));


    let coin = Coin::Dime(22);
    println!("Penny: {:?}", Coin::Penny);
    println!("Dime with value: {:?}", Coin::Dime(31));
    match coin {
        Coin::Penny => println!("It's a penny!"),
        Coin::Nickel => println!("It's a nickel!"),
        Coin::Dime(state) => println!("It's a dime! {}", state),
        Coin::Quarter => println!("It's a quarter!"),
        _ => println!("It's some other coin!"),
    }

    let mut five = Some(5);
    match five {
        None => print!("no value\n"),
        // Some(x) => {
        //     println!("The value is: {}", x);
        //     Some(x + 1)
        // },
        other => println!("The value is"),
    };
    print!("five: {:?}\n", five);

    // let str = Some(String::from("Hello"))
    // match str {
    //     Some(_) => println!("String value!"),
    //     None => println!("No string value"),
    // };
    // println!("str: {:?}\n", str);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    let penny = Coin::Penny;
    let Coin::Dime(value) = penny else{
        println!("Not a dime!");
        return;
    };
    println!("Dime value extracted using let else: {}", value);
}
