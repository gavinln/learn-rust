fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(5);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i64) {
    println!("The value of x is: {} and y is {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
