fn main() {
    let s1 = String::from("hello");
    println!("s1 is {}", s1);

    let s2 = "initial contents".to_string();
    println!("s2 is {}", s2);

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("s3 is {}", s3);

    let mut s4 = String::from("lo");
    s4.push('l');
    println!("s4 is {}", s4);

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");

    let s7 = s5 + &s6;
    println!("s7 is {}", s7);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    for c in "hello".chars() {
        println!("{}", c);
    }

    for b in "hello".bytes() {
        println!("{}", b);
    }

}
