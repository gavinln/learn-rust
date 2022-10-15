#[allow(unused_variables)]

// https://www.youtube.com/watch?v=hEhnDRR4Ijs
fn main() {
    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);

    some_procedure(2.3, 1);

    some_str_procedure("test");

    let string_slice_var: &str = "Howdy!";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a String");
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    // cannot use moved string_var value
    // some_string_procedure(string_var);

    let string_var2 = String::from("I'm a String2");
    some_string_procedure2(&string_var2);
    some_string_procedure2(&string_var2);
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("In function");
    if param_a < 100.0 {
        let return_var = 10.1 * param_a * param_b as f32;
        return_var
    } else {
        -1.
    }
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("In procedure with a {} b {}", param_a, param_b)
}

fn some_str_procedure(param: &str) {
    println!("In some_str_procedure param {}", param)
}

fn some_string_procedure(param: String) {
    println!("In some_string_procedure param {}", param)
}

fn some_string_procedure2(param: &String) {
    println!("In some_string_procedure2 param {}", param)
}
