use std::any::type_name;
use std::mem::size_of;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


fn main() {
    println!("Hello, world!");

    let _guess: u32 = "42".parse().expect("Not a number!");

    // rust integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';  // u8 only

    println!("decimal {}, hex {}, octal {}, binary {}, byte {}",
             decimal, hex, octal, binary, byte);

    let overflow_int1: u8 = "255".parse().expect("Not a number!");
    println!("may overflow {}", overflow_int1);
    // let overflow_int2: u8 = "256".parse().expect("Not a number!");
    // println!("will overflow {}", overflow_int2);

    // floating point variables
    let x = 2.0;
    let y: f32 = 3.0;

    println!("type of x {}, type of y {}", type_of(x), type_of(y));

    // numeric operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // boolean type
    let _true = true;
    let _false: bool = false;

    // character type
    let _character = 'z';
    println!("Size of {}", size_of::<char>());

    // Compound types

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is {} {} {}", x, y, z);
    println!("The value of tup is {} {} {}", tup.0, tup.1, tup.2);

    // array type
    let arr1 = [1, 2, 3];
    println!("The value of arr1 is {} {} {}", arr1[0], arr1[1], arr1[2]);

    let arr2: [i32; 2] = [1, 2];
    println!("The value of arr2 is {} {}", arr2[0], arr2[1]);

    let arr3 = [3; 2];
    println!("The value of arr3 is {} {}", arr3[0], arr3[1]);
}
