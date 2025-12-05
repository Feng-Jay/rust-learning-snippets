fn main() {
    println!("Hello, world!");
    let m1 = String::from("hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{}, {}!", m1, m2);
    println!("{}", s);

    let mut v = vec![1, 2, 3];
    let num = &v[2];
    println!("The third element is: {}", num);
    v.push(4);

    let num = &mut v[2];
    *num += 1;
    println!("The modified third element is: {}", num);
    println!("The vector is now: {:?}", v);

    let mut s = String::from("hello");
    let r1 = &s;
    let s3 = &mut s;
    s3.push_str(", world");
    println!("{}", r1);
}

fn greet(g1: &String, g2: &String){
    println!("Greetings, {} and {}!", g1, g2);
}
