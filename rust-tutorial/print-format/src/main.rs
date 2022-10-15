#[derive(Debug)]
struct DougsData {
    pub a: i32,
    pub b: f64,
}
fn main() {
    println!("Howdy");
    println!("Partner");

    print!("Howdy");
    print!("Partner");

    println!("My data is {}", 5);

    let more_data = 6.7;
    println!("My data is {1} and {0}", 5, more_data);

    println!(
        "My name is {first_name} {last_name}",
        first_name = "Adam",
        last_name = "Smith"
    );

    let data = DougsData { a: 1, b: 1.1 };

    println!("Doug's data is {:?}", data);
    println!("Doug's data is {:#?}", data); // pretty print

    let other_data = DougsData { a: 2, b: 2.2 };

    println!(
        "Doug's data is {0:#?} and other is {1:#?}",
        data, other_data
    );
    let some_formatted_string = format!(
        "Doug's data is {0:#?} and other is {1:#?}",
        data, other_data
    );
    println!("{}", some_formatted_string);
}
