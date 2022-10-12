// Rust for the impatient video
// https://www.youtube.com/watch?v=br3GIIQeefY
fn main() {
    println!("Hello, world!");
    let _x = 42; // _ indicates variable not used
    let _y: i32 = 42;
    let _ = 42;

    let pair = ('a', 17); // tuples
    pair.0;
    pair.1;
    let pair: (char, i32) = ('a', 17);
    pair.0;
    pair.1;

    let (some_char, some_int) = ('a', 17); // destructure tuples
    assert!(some_char == 'a');
    assert!(some_int == 17);

    let slice = [1, 2, 3];
    let (l, r) = slice.split_at(2);
    println!("{:?} {:?}", l, r);

    let (_, right) = slice.split_at(2);
    println!("{:?}", right);

    // multi-line statements
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{:?}", x);

    // shadown variables
    let x = "out";
    {
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
    // expressions: block, function, if/match statements
    let x = {
        let y = 1;
        let z = 2;
        y + z // this is the tail
    };
    println!("{}", x);
    fn fair_dice_roll1() -> i32 {
        return 4;
    }
    fn fair_dice_roll2() -> i32 {
        4
    }
    fair_dice_roll1(); // this and the function below are equivalent
    fair_dice_roll2();
    fn fair_dice_roll3() -> i32 {
        if true {
            6
        } else {
            4
        }
    }
    println!("{}", fair_dice_roll3());
    fn fair_dice_roll4() -> i32 {
        match true {
            true => 6,
            false => 4,
        }
    }
    println!("{}", fair_dice_roll4());

    // use a dot to access components
    let a = (10, 20);
    a.0; // == 10
    struct Name {
        nickname: String,
        name: String,
    }
    fn get_some_struct() -> Name {
        Name {
            nickname: String::from("fasterthanlime"),
            name: String::from("amos"),
        }
    }
    let amos = get_some_struct();
    amos.nickname;
    amos.name;

    let nick = "fasterthanlime";
    println!("{}", nick.len());

    let least = std::cmp::min(3, 8);
    println!("{}", least);

    let x = "amos".len(); // 4
    println!("{}", x);
    let x = str::len("amos"); // 4
    println!("{}", x);

    // declare and initialize structs
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Number {
        odd: bool,
        value: i32,
    }
    let x = Number {
        odd: false,
        value: 2,
    };
    let y = Number {
        value: 3,
        odd: true,
    };
    println!("{:?}", x);
    println!("{:?}", y);

    // match patterns
    fn print_number(n: Number) {
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", n.value),
        }
    }
    print_number(x);

    // methods for structs
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("{}", minus_two.is_positive());
    // immutable by default
    // y.odd = true;  // cannot assign to y.odd

    // mutable variables
    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 19;

    // generic functions
    #[allow(dead_code)]
    fn foobar1<T>(_: T) {}
    #[allow(dead_code)]
    fn foobar2<L, R>(_: L, _: R) {}

    // generic structs
    struct Pair<T> {
        a: T,
        b: T,
    }
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    println!("{} {}", p1.a, p1.b);
    println!("{} {}", p2.a, p2.b);

    // standard library vectors are generic types
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    println!("{:?}", v1);
    println!("{:?}", v2);

    // vector literals
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];
    println!("{:?}", v1);
    println!("{:?}", v2);

    // macros
    // panic!("This stops execution with file name and line number")

    // some methods panic
    let o1: Option<i32> = Some(128);
    println!("{:?}", o1.unwrap());
    let o2: Option<i32> = None;
    println!("{:?}", o2);
    // o2.unwrap(); // this panics!

    // Option enum
    #[allow(dead_code)]
    enum Option2<T> {
        None,
        Some(T),
    }

    impl<T> Option2<T> {
        fn unwrap(self) -> T {
            match self {
                Self::Some(t) => t,
                Self::None => panic!(),
            }
        }
    }
    let o3: Option2<i32> = Option2::Some(256);
    println!("{}", o3.unwrap());

    // Result enum
    #[allow(dead_code)]
    enum Result2<T, E> {
        Ok(T),
        Err(E),
    }

    // functions that fail typically return Result
    let s1 = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s1);

    #[allow(unused_variables)]
    let s2 = std::str::from_utf8(&[195, 40]); // 195 is invalid
                                              // calling unwrap on s2 will panic

    // let s3 = std::str::from_utf8(&[195, 40]).expect("valid utf-8");

    // match to handle error
    let melon = &[240, 159, 141, 137];
    match std::str::from_utf8(melon) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{}", e),
    }

    // if let handles non-error values
    if let Ok(s) = std::str::from_utf8(melon) {
        println!("{}", s);
    }

    // let s = std::str::from_utf8(melon)?; // return Error

    // iterators
    // let natural_numbers = 1..; // infinte iterator
    println!("{}", (0..).contains(&100)); // true
    println!("{}", (..=20).contains(&20)); // true
    println!("{}", (3..6).contains(&4)); // true
    println!("{}", (3..6).contains(&6)); // false

    // vector/slice/iterator in for loops
    for i in vec![52, 49, 21] {
        println!("I like number {}", i);
    }
    for i in &[52, 49, 21] {
        println!("I like number {}", i);
    }
    for c in "rust".chars() {
        println!("Give me a {}", c);
    }
    // fluent pattern
    for c in "SuRPRISE INbOUND"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        print!("{}", c);
    }
}
