use::std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("Counter: {}", *num);
            // thread::sleep(std::time::Duration::from_secs(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // dead lock example
    let lock1 = Arc::new(Mutex::new(("lock1")));
    let lock2 = Arc::new(Mutex::new(("lock2")));
    let mut handles = vec![];

    // thread#1
    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);
    let handler = thread::spawn(move || {
        println!("Thread {} attempting to acquire locks", 1);
        let _a = l1.lock().unwrap();
        println!("Thread {} acquired first lock", 1);
        
        thread::sleep(std::time::Duration::from_millis(1000));

        println!("Thread {} attempting to acquire second lock", 1);
        let _b = l2.lock().unwrap();
        println!("Thread {} acquired both locks", 1);
    });
    handles.push(handler);

    // thread#2
    let handler = thread::spawn(move || {
        println!("Thread {} attempting to acquire second lock", 2);
        let _b = lock2.lock().unwrap();
        println!("Thread {} acquired second locks", 2);

        thread::sleep(std::time::Duration::from_millis(1000));
        
        println!("Thread {} attempting to acquire first lock", 2);
        let _a = lock1.lock().unwrap();
        // thread::sleep(std::time::Duration::from_millis(1000));
        println!("Thread {} acquired both lock", 2);
    });
    handles.push(handler);

    for handle in handles {
        handle.join().unwrap();
    }

}
