use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael, Some years ago...");
    let i;
    {
        let first_sentence = novel.split('.').next().expect("Count not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("The first sentence is {}", i.part);
    println!("level is {}", i.level());
    println!("return part is {}", i.announce_ad_return_part("hello"));

    let s: &'static str = "I have a static lifetime";
    println!("String s is {}", s);

    let result = longest_with_an_annoucement(string1.as_str(), string2.as_str(), "hello");
    println!("result is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y.clone()
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_ad_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
