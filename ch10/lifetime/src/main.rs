use std::result;

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    // let test = String::from("test");
    if x.len() > y.len() {
        x
    } else {
        // &test
        y
    }
}

fn longest2<'a, 'b> (x: &'a str, y: &'b str) -> &'a str {
    // let test = String::from("test");
    if x.len() > y.len() {
        x
    } else {
        // y // <- error
        x
    }
}

struct Refereces<'a>{
    r: &'a str,
}

impl<'a> Refereces<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.r
    }
}


fn main() {
    // let r;
    // {
    //     let x = 5;
        // r = &x; // <- dangling reference
    // }
    // println!("r: {}", r);
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }
    // println!("The longest string is {}", result);

    println!("Longest2: {}", longest2("hello", "hi"));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = Refereces { r: first_sentence  };
    println!("First sentence: {}", i.r);

}
