// destructuring
#[allow(dead_code)]
fn main() {
    // tuples
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
        (1, ..) => println!("First is 1, rest does not matter"),
        (.., 2) => println!("Last is 2, rest does not matter"),
        (3, .., 4) => println!("First is 3, last is 4, rest does not matter"),
        _ => println!("It doesn't matter what they are"),
    }
    // arrays/slices
    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!("array[1] = {}, array[3] = {}", second, third),
        [1, _, third] => println!("array[2] = {}", third),
        [-1, second, ..] => println!("array[1] = {}", second),
        [3, second, tail @ ..] => println!("array[1] = {}, others were {:?}", second, tail),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, last = {:?}",
            first, middle, last
        ),
    }
    // enums
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
    // pointers/ref
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 4;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. mut_value: {:?}", m);
        }
    }

    // structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}
