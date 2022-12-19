#[allow(unused)]
#[allow(unused_mut)]
fn main() {
    // signed integers: i8, i16, i32, i64, i128, isize (pointer size)
    // unsigned integers: u8, u16, u32, u64, u128, usize (pointer size)
    // floating point: f32, f64
    // char: Unicode scalar values - 4 bytes each
    // bool either true or false
    // unit type ()

    // Compound typed
    // arrays like [1, 2, 3]
    // tuples like (1, true)

    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12;
    inferred_type = 234234234;

    let mut mutable = 12;
    // expected integer, found bool
    // mutable = true;

    // can shawdow variable
    let mutable = true;
}
