use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("Hello, world!");
    // Box<T> used when
    // 1. type whose size can't be known at compile time
    // 2. large amount of data should not be copied by ownership transferred
    // 3. when a type should implement a trait rather than being of a specific type

    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}
