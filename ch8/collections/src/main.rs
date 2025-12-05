use std::{collections::{HashMap, hash_map}, ptr::hash};

fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let mut v = vec![5,1,3,2,4];
    v.sort();
    v.push(1);
    let third = &mut v[2];
    *third = 10;
    // println!("The third element is: {}", third);
    for i in &mut v {
        // println!("{}", i);
        *i += 1;
    }
    println!("{:?}", v);

    let mut v2 = Vec::new();
    let s = String::from("value");
    v2.push(s);
    // println!("{:?}", s);
    // let mut s = v2[0];

    // for n_ref in v.iter() {
        // println!("{}", n_ref);
        // v.push(*n_ref);
    // }

    let mut iter = v.iter();
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap());

    let mut iter_num = 0 .. v.len();
    let i1 = iter_num.next().unwrap();
    let i2 = iter_num.next().unwrap();
    println!("{}, {}", i1, i2);

    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
    // let c = *v2[1]; // when the mutable borrow occurs, the immutable borrow is still active

    let init_data = "here is a str";
    let mut s = init_data.to_string();
    s.push_str("string");
    s.push('c');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
    // let c = s3[0];
    let hello = "Здравствуйте你好好";

    let s = &hello[0..4];
    println!("{}", s);
    for item in hello.chars() {
        println!("{}", item);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 50);
    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let new_key = String::from("green");
    let new_v = 30;
    scores.insert(new_key, new_v);
    // println!("{}", new_key); // new_key is moved
    println!("{}", new_v); // new_v is copied

    {
        let new_key = String::from("yellow");
        let new_v = 40;
        scores.insert(new_key, new_v);
    }
    

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    let mut scores = HashMap::new();
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    let test = scores.entry(String::from("Yellow")).or_insert(50);
    *test += 10;
    println!("{:?}", scores);
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{}",map["world"]);
    println!("{:?}", map);
}