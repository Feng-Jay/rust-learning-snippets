fn main() {
    let x = 5;
    let y = 32u8;
    let x = x + 1;
    println!("the value of y is: {y}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup: (i32, u8, f64) = (50000, 66, 3.14);
    let test = tup.0;
    println!("{test}");

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("The value of x is: ({}, {})", x.0, x.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", a[0]);
    let a = [3; 5];
    println!("The first element of the array is: {}", a[0]);
}