// for keyword
fn main() {
    // n will take the values 1, 2, ..., 100
    // for n in 1..=100 {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    // for and iterators
    let names1 = vec!["Bob", "Frank", "Ferris"];
    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names1: {:?}", names1);

    // consuing iterators
    let names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }
    // Borrow of moved value
    // println!("names2: {:?}", names2);

    // mtably borrowed and iterators
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean",
            _ => "Hello",
        }
    }
    println!("names3: {:?}", names3);
}
