// diverging functions
fn main() {
    // function never returns
    fn _foo() -> ! {
        panic!("This call never returns.")
    }
    fn some_fn() {
        ()
    }

    let a: () = some_fn();
    println!("a is {:?}", a);

    let p_fn: ! = panic!("This call never returns.");
    p_fn();
}
