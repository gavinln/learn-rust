#[allow(unused_variables)]
#[allow(unused_assignments)]

// https://www.youtube.com/watch?v=1nCZOh83ins

fn main() {
    let some_data: bool = true; // or false
    println!("{}", some_data);

    let some_data: i8 = 10;
    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);

    // 130 is greater than max
    // will panic in debug model: cargo run
    // wrong result in release model: cargo run -r
    // let new_data = some_data + 120;
    // println!("{:?}", new_data);

    let some_data: u8 = 10;
    println!("Min u8 is {}", std::u8::MIN);
    println!("Max u8 is {}", std::u8::MAX);

    // size of default computer int type
    let somme_isize: isize = 10;
    println!("Min isize is {}", std::isize::MIN);
    println!("Max isize is {}", std::isize::MAX);

    let some_data: f32 = 10.; // need decimal point    println!("Min f32 is {}", std::f32::MIN);
    println!("Max f32 is {}", std::f32::MAX);

    let some_char: char = 'a'; // 4 bytes
    println!("{}", some_char);
}
