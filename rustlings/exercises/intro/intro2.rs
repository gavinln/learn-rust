// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

fn main() {
    let name1: String = "World".to_string();
    println!("Hello {}!", name1);
    let name2: &str = "World";
    println!("Hello {}!", name2);
}
