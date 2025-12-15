use std::io::Read;
fn foo(x: i32) -> i32 {
   if x == 0 {
    1
   }else if x < 0 {
    -1
   }else {
    1
   }
}


fn bar() -> Result<String, std::io::Error> {
    // read from file
    let mut file = std::fs::File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains("apple"){
        println!("Found apple!");
        Ok(contents)
    } else {
        println!("No apple found.");
        Ok(String::from("Maybe banana"))
    }
}



fn main() {
    let res = foo(10);
    println!("Result: {}", res);
    bar();
}


