fn main() {
    println!("Hello, world!");
    // line commments
    /* Block comments
     *
     */
    println!("{} days", 31);

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10: {}", 69420);
    println!("Base 2 binary: {}", 69420);
    println!("Base 8 octal: {}", 69420);
    println!("Base 16 hexadecimal: {:x}", 69420);
    println!("Base 16 hexadecimal: {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    #[allow(dead_code)]
    struct Structure(i32);

    // will not compile as fmt::Display not implemented
    // println!("This struct {} won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
