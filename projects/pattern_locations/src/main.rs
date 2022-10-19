fn main() {
    // match arms
    let x = Some(4);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("{:?}", y);

    // if let expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color} as the background")
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // for loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    let (x, y, z) = (1, 2, 3);
    println!("{} {} {}", x, y, z);

    // function parameters
    let point = (3, 3);
    print_coordinates(&point);

    // Refutable patterns can fail to match
    // e.g. if let Some(x) = value can fail to mach
    // if value is None rather than Some

    // Irrefutable patterns will always match
    // let statements and for loops only accept irrefutable patterns

    // match arms must use refutable patterns, except for the last arm
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y)
}
