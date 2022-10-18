use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // mutex is an abbrevation for mutual exclusion
    // mutex allows only one thread ot access some data at a time
    // To use a mutex
    // 1. attempt to acquire the lock before using the data
    // 2. when done with the mutex guarded data, unlock the mutex

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
