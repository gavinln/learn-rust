// https://www.youtube.com/watch?v=nvur2Ast8hE

// struct Point {
//     x: i32,
//     y: i32,
// }

#[allow(unused_variables)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
#[derive(Debug)]
enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC,
}

#[allow(dead_code)]
fn dougs_func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug>(
    input_a: T,
    input_b: T,
) -> T {
    println!("input_a has {:?}", input_a);
    input_a - input_b
}

#[allow(dead_code)]
fn dougs_func2<T, E>(input_a: T, input_b: T, input_e: E) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
    E: std::fmt::Debug,
{
    println!("input_a has {:?}", input_a);
    println!("input_a has {:?}", input_e);
    input_a - input_b
}

trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct DougsStruct {
    something: i32,
}

impl SomeCustomTrait for DougsStruct {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

#[allow(dead_code)]
fn do_this<T>(some_var: &T) -> String
where
    T: SomeCustomTrait + std::fmt::Debug,
{
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}

// fn do_this2(some_var: &dyn SomeCustomTrait) -> String {
//     println!("{:?}", some_var);
//     some_var.blah_blah("first", "second")
// }

#[allow(dead_code)]
struct DougsStruct2<T, U> {
    dougs_t: T,
    dougs_u: U,
}

impl<T, U> DougsStruct2<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn log_something(&self) {
        println!("{:?} {:?}", self.dougs_t, self.dougs_u);
    }
}

fn main() {
    let a = Point {
        x: 100,
        y: -1 as f32,
    };
    println!("x = {} y = {}", a.x, a.y);

    let b = Point { x: 10.1, y: 2.3 };
    println!("x = {} y = {}", b.x, b.y);

    let some_data = SomeEnum::OptionA(34.2);
    println!("{:?}", some_data);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
        SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
        SomeEnum::OptionC => println!("OptionC"),
    }

    let some_data2 = SomeEnum::OptionB('b');
    println!("{:?}", some_data2);

    let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);
    println!("{:?}", some_data3);

    let a = dougs_func(4, 5);
    println!("a has {}", a);

    let test = DougsStruct { something: 1000 };
    let result = do_this(&test);
    println!("result {}", result);

    let testi32 = 10;
    let result2 = do_this(&testi32);
    println!("result2 {}", result2);

    let test2 = DougsStruct2 {
        dougs_t: 5.6,
        dougs_u: vec![1, 2, 3],
    };
    test2.log_something();
}
