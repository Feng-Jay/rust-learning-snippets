static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 20;
    let r1 = &raw const num;
    let r2 = &raw mut num;
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
    println!("r1: {:?}, r2: {:?}", r1, r2);
    unsafe {
        println!("r1: {:?}, r2: {:?}", *r1, *r2);
        println!("COUNTER: {}", *(&raw const COUNTER));
        add_to_count(5);
        // println!("COUNTER: {}", COUNTER);
        // why raw pointer? The references are checked by borrow checker, there is no way a mutable and immutable reference coexist.
        // however, these two raw pointers can coexist.
    }
}
