// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct SomeStruct {
    num: i32,
}

fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.num > b.num {
        a
    } else {
        b
    }
}

#[allow(unused)]
fn lifetime_example() {
    let some_struct = SomeStruct { num: 3 };
    let bigger: &SomeStruct;
    let other_struct = SomeStruct { num: 5 };
    bigger = biggest(&some_struct, &other_struct);
    println!("{:?}", &bigger);
}

#[allow(unused)]
fn references_example() {
    let mut blue_str = String::from("Blue");
    let ref1 = &blue_str;
    println!("{:?}", ref1);
    blue_str.push_str(" color");
    println!("{:?}", blue_str);
    // immutable borrow after using mutable reference
    // println!("{:?}", ref1);
}

fn temp() -> Box<i32> {
    let val = Box::new(3);
    println!("{}", val);
    val
}

fn main() {
    let a = temp();
    println!("{}", a);

    // references_example();
    // lifetime_example();
}
