#[allow(unused_variables)]

// https://www.youtube.com/watch?v=Vz9_y7NjRlQ

fn main() {
    let some_bool = true;
    let some_int = 30;
    let some_int2 = 50;

    if (some_bool == true || some_int > 100) && some_int2 == 200 {
        println!("If true branch")
    } else if some_int == 30 {
        println!("Else if branch")
    } else {
        println!("Else branch")
    }

    let var_from_inline = if some_int == 9 { 300 } else { 400 };

    match some_bool {
        true => println!("true branch"),
        false => println!("false branch"),
    }

    match some_int {
        0 => println!("0 branch"),
        1..=100 => println!("between1 and 100 branch"),
        _ => println!("else branch"),
    }

    let var_from_match = match some_bool {
        true => 10,
        false => 20,
    };

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };
}
