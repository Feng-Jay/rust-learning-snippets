
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    println!("Hello, world!");
    let user = User{
        username: String::from("testuser"),
        email: String::from("testemail"),
        sign_in_count: 1,
        active: true,
    };
    println!("Username: {}", user.username);
    let mut user2 = build_user(user.username, user.email);
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    println!("User3 Email: {}", user3.email);
    // println!("Updated Email: {}", user2.username); briefly, update is like the `=` operator
    
    let black = Color(0, 0, 0);
    println!("Black color RGB: ({}, {}, {})", black.0, black.1, black.2);
    let tuple = (0, 0 ,0);
    let (x, y, z) = tuple;
    println!("Tuple values: ({}, {}, {})", x, y, z);
    let Color(r, g, b) = black;
    println!("Destructured Color RGB: ({}, {}, {})", r, g, b);
    
}
