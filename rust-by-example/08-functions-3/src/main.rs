// closures
fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let a = 1;
    println!("function {}", function(a));
    println!("closure annotated {}", closure_annotated(a));
    println!("closure inferred {}", closure_inferred(a));

    // closure taking no arguments
    let one = || 1;
    println!("closure returning one: {}", one());

    // capturing
    use std::mem;

    let color = String::from("green");

    // immutable reference borrow
    let immutable_borrow = || println!("color: {}", color);
    immutable_borrow();

    let _reborrow = &color;
    immutable_borrow();

    // can move after final use of immutable_borrow()
    let _color_moved = color;

    let mut count = 0;
    let mut mutable_borrow = || {
        count += 1;
        println!("count: {}", count)
    };
    mutable_borrow();
    // cannot borrow as immutable
    // let _reborrow = &count;
    mutable_borrow();

    let _count_reborrowed = &mut count;

    // this is a non-copy type and needs to be moved
    let movable = Box::new(3);

    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable)
    };
    consume();
    // can only be called once
    // consume();
}
