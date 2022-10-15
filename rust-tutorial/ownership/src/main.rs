#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    let var_a = String::from("Howdy");
    let var_b = var_a;

    // println!("{}", var_a)

    // stack variables are fixed size
    let stack_i8: i8 = 10;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    if stack_i8 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    }

    // heap variables can grow in size
    // memory can live beyond scope that created it

    let heap_vector: Vec<i8> = Vec::new();
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

    let mut stack_i8_2 = stack_i8;
    stack_i8_2 = 33; // only stack_i8_2 is changed not stack_i8
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);

    let heap_i8_2 = heap_i8;
    // heap_i8 is moved and cannot be used
    // println!("{}", heap_i8);
    println!("{}", heap_i8_2);

    let heap_i8: Box<i8> = Box::new(30);
    let heap_i8_2 = heap_i8.clone(); // clones instead of moving ownership
    let heap_i8_2 = Box::new(33);
    println!("{}", heap_i8);
    println!("{}", heap_i8_2);

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("In main stack {}", heap_f64);

    let some_string: String = String::from("Howdy");
    let some_str: &str = "Partner";

    some_procedure(&some_string, some_str);
    println!("{} {}", some_string, some_str);

    // only immutable references
    let var_a = String::from("Howdy!");
    let var_b = &var_a;
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);

    // mutable ref used after immutable ref created & used
    let mut var_d = String::from("Howdy!");
    let var_e = &var_d;
    let var_f = &var_d;
    println!("{} {} {}", var_d, var_e, var_f);
    var_d.push('a');

    // mutable ref used before immutable ref created & used
    let mut var_g = String::from("Howdy!");
    var_g.push('a');
    let var_h = &var_g;
    let var_i = &var_g;
    println!("{} {} {}", var_g, var_h, var_i);

    // mutable ref used BETWEEN immutable ref created & used
    #[allow(unused_mut)]
    let mut var_j = String::from("Howdy!");
    let var_k = &var_j;
    let var_l = &var_j;
    // cannot borrow as mutable as also borrowed as immutable
    // var_j.push('a');
    println!("{} {} {}", var_j, var_k, var_l);

    let var_struct = DougsStruct { a: 9, b: 10. };
    some_struct_procedure(&var_struct);
    println!("{:?}", var_struct);

    some_struct_procedure2(var_struct.clone());
    println!("{:?}", var_struct);

    let mut var_struct3 = DougsStruct { a: 9, b: 10. };
    some_struct_procedure3(&mut var_struct3);
    println!("{:?}", var_struct3);
}

#[derive(Debug, Clone)]
struct DougsStruct {
    a: i32,
    b: f64,
}

fn some_struct_procedure(param: &DougsStruct) {
    println!("{} {}", param.a, param.b);
}

fn some_struct_procedure2(param: DougsStruct) {
    println!("{} {}", param.a, param.b);
}

fn some_struct_procedure3(param: &mut DougsStruct) {
    param.a += 10;
    println!("{} {}", param.a, param.b);
}

fn some_procedure(param_a: &String, param_b: &str) {
    println!("{} {}", param_a, param_b);
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
    println!("In heap_procedure with param {}", *param);
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn _main() {
    let a = Box::new(3.);
    let b = Box::new(4.);
    _print_type_of(&a);
    _print_type_of(&b);
    let c = Box::new(*a + *b);
    _print_type_of(&c);
    println!("{} {} {}", a, b, c);
}
