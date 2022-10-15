fn main() {
    let mut s = String::from("hello world");
    let word = first_word_index(&s);  // word will have the value 5
    println!("s has value '{}'", s);
    s.clear();
    // word still has a value even if s is cleared
    println!("word has value {}", word);

    let s1 = String::from("hello");

    let slice1 = &s1[0..2];
    let slice2 = &s1[..2];
    println!("slice1 is {}, slice2 is {}", slice1, slice2);

    let len = s1.len();
    let slice1 = &s1[3..len];
    let slice2 = &s1[3..];
    println!("slice1 is {}, slice2 is {}", slice1, slice2);

    let slice1 = &s1[0..len];
    let slice2 = &s1[..];
    println!("slice1 is {}, slice2 is {}", slice1, slice2);

    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();  // cannot mutable borrw because it is borrowed as immutable
    println!("word has value {}", word);

    // array slices

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}
