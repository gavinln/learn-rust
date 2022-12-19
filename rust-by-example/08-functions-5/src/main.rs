// higher order functions

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // imperative approach
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("Imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|x| x * x)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("Functional style: {}", sum_of_squared_odd_numbers);
}
