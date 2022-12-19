// declare first and freezing
fn main() {
    // declare a variable before initializing
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let mut _mutable_integer = 7i32;
    {
        // shadowing by immutable
        let _mutable_integer = _mutable_integer;
        // cannot modify _mutable_integer here
    }
    _mutable_integer = 3;
    println!("_mutable_integer: {}", _mutable_integer);
}
