fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    // let x = (let y = 10); let is not an expression
    // while block is
    let x = {
        let y = 10;
        y + 5
    };
    println!("The value of x is: {x}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x plus one is: {x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("This is another function.");
    println!("The value of x is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}