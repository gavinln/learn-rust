// closure input and output parameters
fn main() {
    // Fn: closure captures value by reference
    // FnMut: closure uses captured value by mutable reference
    // FnOnce: closure captures value by value

    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    fn apply2<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // capture 2 variables
    // greeting by reference and farewell by value
    let diary = || {
        println!("greeting is {}", greeting);

        // modification requires FnMut
        farewell.push_str("!!!");
        println!("farewell is {}", farewell);

        // dropping forces FnOnce
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply2(double));

    // closures as output parameters
    // The move keyword must be used, which signals that all captures
    // occur by value. This is required because any captures by reference
    // would be dropped as soon as the function exited, leaving invalid
    // references in the closure.

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
