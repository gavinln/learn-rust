// https://www.youtube.com/watch?v=1QoT9fmPYr8
// lifetimes used to prevent references
// going out of scope before being used
#[allow(dead_code)]
const SOME_INT: i32 = 20;
const SOME_CONST_A: &str = "i'm a constant!";
const SOME_CONST_B: &str = "i'm a constant, too!";

#[allow(dead_code)]
struct DougsStruct<'a, 'b: 'a> {
    some_data: Vec<i32>,
    some_reference_data: &'a Vec<i32>,
    some_reference_data2: &'b Vec<i32>,
}

#[allow(unused_variables)]
fn main() {
    let t1 = vec![1., 2.];

    let a;
    // works fine as ownership is transferred
    {
        let b = String::from("Howdy");
        a = b // ownership transferred
    }
    println!("{}", a);

    let c: &String;
    // deos not work as reference to out of scope variable
    {
        let d = String::from("Howdy");
        // c = &d // reference to d which goes out of scope
    }
    // println!("{}", c);

    let var1 = 10;
    let var2 = 20;
    let result_ref = get_int_ref(&var1, &var2);
    println!("{}", result_ref);

    let result_ref = get_int_ref2(&var1, &var2);
    println!("{}", result_ref);

    let a = String::from("hello");
    let b = String::from("hi");

    let c = get_str_ref(&a, &b);
    println!("{}", c);

    let some_vec = vec![1, 2, 3];
    let result = get_vec_slice(&some_vec);
    println!("{:?}", result);

    let other_vec = vec![1, 2];
    let result = get_vec_slice2(&some_vec, &other_vec);
    println!("{:?}", result);

    println!("{:?}", some_func());
    println!("{:?}", some_func2(SOME_CONST_A, SOME_CONST_B));

    println!("{}", get_smaller(&3, &4));
    println!("{}", get_smaller(&5., &3.));
    println!("{}", get_smaller(&"a", &"ab"));
}

fn get_smaller<'a, T>(param_1: &'a T, param_2: &'a T) -> &'a T
where
    T: std::cmp::PartialOrd,
{
    if param_1 < param_2 {
        param_1
    } else {
        param_2
    }
}

fn some_func() -> &'static str {
    SOME_CONST_A
}

fn some_func2(param_1: &'static str, param_2: &'static str) -> &'static str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

fn get_vec_slice(param_1: &[i32]) -> &[i32] {
    &param_1[0..2]
}

fn get_vec_slice2<'a>(param_1: &'a [i32], param_2: &'a [i32]) -> &'a [i32] {
    if param_1.len() > param_2.len() {
        &param_1[0..2]
    } else {
        &param_2[0..2]
    }
}

// lifetimes don't apply because no reference inputs/outputs
#[allow(dead_code)]
fn test_1(param_1: Vec<f64>) -> Vec<f64> {
    param_1
}

// lifetimes not an issue as there is not reference output
#[allow(dead_code)]
fn test_2(param_1: &Vec<f64>) -> Vec<f64> {
    param_1.clone()
}

// same lifetime applied to output and input automatically
#[allow(dead_code)]
fn test_3(param_1: &Vec<f64>) -> &Vec<f64> {
    param_1
}

fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

fn get_int_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        &param_1
    }
}

fn get_int_ref2<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        &param_1
    }
}
