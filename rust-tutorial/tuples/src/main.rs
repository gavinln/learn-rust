#[allow(unused_variables)]

fn main() {
    let some_tuple = (2, 3.4, ("a", "b"));

    println!(
        "tuple data is {} {} {} {}",
        some_tuple.0,
        some_tuple.1,
        some_tuple.2 .0,
        (some_tuple.2).0
    );
    println!("tuple data is {:?}", some_tuple);

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);

    let (my_red, my_green, my_blue) = some_color;
    println!("r {}, g {}, b {}", my_red, my_green, my_blue);

    // r g b a - red, green, blue, alpha
    let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);

    let empty_tuple = ();

    match some_color.2 {
        0..=200 => println!("blah!"),
        _ => (),
    }

    some_procedure()
}

fn get_some_rgb() -> (u8, u8, u8) {
    (200, 100, 20)
}

fn some_procedure() {
    ()
}
