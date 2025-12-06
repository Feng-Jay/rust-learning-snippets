use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello2.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello2.txt")?; // god, so many syntax sugar...
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    // or more shorter:
    // let mut username = String::new();
    // File::open("hello2.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    // or
    // std::fs::read_to_string("hello2.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn std::error::Error>> { //trait object?
    // println!("Hello, world!");
    // panic!("Crash and burn!")
    // let v = vec![1, 2, 3];

    // v[99];
    use std::fs::File;
    let f = File::open("hello.txt"); // Result<File, std::io::Error>
    let _file = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            std::io::ErrorKind:: NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            }
        },
    };

    let _f2 = File::open("hello2.txt").unwrap_or_else(|error|{
        if error.kind() == std::io::ErrorKind::NotFound{
            File::create("hello2.txt").unwrap_or_else(|error|{
                panic!("Tried to create file but there was a problem: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        }
    });
    // let _f3 = File::open("hello3.txt");
    // let _f3 = File::open("hello3.txt").unwrap();
    // let _f3 = File::open("hello3.txt").expect("Failed to open hello3.txt");
    let _f3 = read_username_from_file_shorter();
    println!("{:?}", _f3);

    let res = last_char_of_first_line("");
    println!("{:?}", res);

    Ok(())
}
