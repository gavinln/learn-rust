#[allow(unused_variables)]

static mut MY_STATIC_VARIABLE: i32 = 10;

fn main() {
    // casting
    let some_i32: i32 = 10;
    let some_f64: f64 = 20.2;

    let combined = some_i32 as f64 + some_f64;
    println!("{}", combined);

    // shadowing
    let var_a: i32 = 10;
    {
        println!("inner scope can see outer scope {}", var_a);
        let var_a: f32 = 20.2222;
        println!("inner scope can see nnner scope {}", var_a);
    }
    println!("outer scope variable {}", var_a);

    // const
    const DOUGS_CONSTANT: i64 = 100;
    println!("Doug's constant is {}", DOUGS_CONSTANT);

    let circle_pi = std::f32::consts::PI;
    println!("{}", circle_pi);

    // static
    unsafe {
        MY_STATIC_VARIABLE = 20;
        println!("{}", MY_STATIC_VARIABLE);
    }
}
