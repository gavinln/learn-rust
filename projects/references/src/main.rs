fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("Changed s2 is '{}'", s2);

    let r1 = &s1;
    let r2 = &s1;
    println!("s1 is {}, r1 is {}, r2 is {}", s1, r1, r2);

    let mut s3 = String::from("hello");
    let r3 = &mut s3;
    println!("r3 is {}", r3);
    // cannot use both s3 and r3 as both are mutable
    // println!("s3 is {}, r3 is {}", s3, r3);

    // two mutable references used sequentially
    {
        let r4 = &mut s3;
        println!("r4 is {}", r4);
    }
    let r5 = &mut s3;
    println!("r5 is {}", r5);

    // let reference_to_nothing = dangle();
    let _reference = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s2: &mut String) {
    s2.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
