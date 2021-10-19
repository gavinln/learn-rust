fn main() {
    let s = "hello";
    // cannot modify s
    // s.push_str(", world!");
    println!("the value if s is: {}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("the value if s is: {}", s);

    let x = 5;
    let y = x;

    println!("The value of x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    // cannot access s1
    // println!("The value of s1 is {}, s2 is {}", s1, s2);
    println!("The value of s2 is {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("The value of s1 is {}, s2 is {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    // cannot access s
    // println!("The value of s is {}", s);

    let s1 = gives_ownership();
    println!("The value of s1 is {}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_ownership(s2);
    println!("The value of s3 is {}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn takes_ownership(s: String) {
    println!("The value of s is {}", s);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_ownership(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
