// From and Into
use std::convert::From;

fn get_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("{}", get_type_of(&my_str));
    println!("{}", get_type_of(&my_string));

    let num = Number::from(30);
    println!("My number is {:?}", num);

    println!("{}", get_type_of(&num));

    let int = 5;
    let num2: Number = int.into();
    println!("My number is {:?}", num2);
}
