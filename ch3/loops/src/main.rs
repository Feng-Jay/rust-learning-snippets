fn main() {
    let mut counter  = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break  counter * 2;
        }
    };
    println!("The result is {}", result); // loop is a statement while break returns a value


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaning = 10;

        loop {
            println!("remaning = {remaning}");
            if remaning == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaning -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    // println!("{:#?}", a[..=3]);

}
