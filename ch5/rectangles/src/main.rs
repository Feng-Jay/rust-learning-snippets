
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(size: u32) -> Self{
        Self { width: size, height: size, }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width, height));

    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect));
    println!("{}", rect.0);
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", area_rectangle(&rect));
    println!("rect is {rect:#?}");
    dbg!(&rect);
    println!("{}", rect.width);

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("The rectangle has a nonzero width: {}", rect.width());
    let mut rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    impl Rectangle {
        fn test(){
            println!("test");
        }
        fn max(&self, other:Rectangle) -> Rectangle{
            Rectangle{
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            }
        }
        fn set_to_max(&mut self, other:Rectangle){
            *self = self.max(other);
            self.width += 10;
        }
    }
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    dbg!(Rectangle::new(40));
    let test = & rect2;
    rect2.set_to_max(Rectangle::new(100));
    dbg!(&test);
    dbg!(&rect2);
}
