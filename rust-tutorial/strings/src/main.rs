#[allow(unused_variables)]

// https://www.youtube.com/watch?v=ClPrjjHmo2Y

fn main() {
    // generally immutable, allocated on heap/stack/in code
    let example_str: &str = "Howdy";
    println!("{}", example_str);

    // Allocated on the heap and mutable
    let example_string: String = String::from("Partner");
    println!("{}", example_string);

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "String literal".to_string();

    let string_from_literal = String::from("String literal");
    let string_from_str_val = String::from(example_str);

    let str_from_string: &str = &example_string;

    // cannot concatenate string literals
    // let combined_string = "first" + "second";

    let combine_string_literals = ["first", "second"].concat();
    println!("{}", combine_string_literals);

    let combine_with_format_macro = format!("{} {}", "first", "second");
    println!("{}", combine_with_format_macro);

    // String has to be on the left
    let string_plus_str = example_string + example_str;
    println!("{}", string_plus_str);

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push('x');
    mut_string.push_str(&String::from("abc"));
    println!("{}", mut_string);

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b; // need reference for second and subsequent
    println!("{}", combined);

    let str_from_substring: &str = &example_str[0..2];
    println!("{}", str_from_substring);
    println!("{}", &example_str[1..]);
    println!("{}", &example_str[..2]);
    println!("{}", &example_str[..=2]);

    // will panic in debug and release
    // println!("{}", &example_str[..=200]);

    // will not panic
    println!("{:?}", &example_str.chars().nth(200)); // returns None

    let char_by_index = &example_str.chars().nth(2);

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {}
    }

    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}
