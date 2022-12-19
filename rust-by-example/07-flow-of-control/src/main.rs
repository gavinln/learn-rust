fn main() {
    // if/else control flow
    let n = 5;

    if n < 0 {
        println!("{} is negative", n)
    } else if n > 0 {
        println!("{} is negative", n)
    } else {
        println!("{} is zero", n)
    }

    let big_n = if n < 10 && n > -10 {
        println!("increase 10 fold");
        10 * n
    } else {
        println!("halve");
        n / 2
    };
    println!("{} to {}", n, big_n);
}
