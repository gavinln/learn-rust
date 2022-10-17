use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!")
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = 5;
    let b = Box::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let c = 5;
    let d = MyBox::new(c);

    assert_eq!(5, c);
    assert_eq!(5, *d);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
