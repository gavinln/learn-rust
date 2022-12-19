// if let and while let
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {}", i);
    }

    if let Some(i) = letter {
        println!("Matched {}", i);
    } else {
        println!("Didn't match a number")
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar")
    }

    if let Foo::Bar = b {
        println!("b is foobar")
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value)
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred")
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {}, incrementing", i);
            optional = Some(i + 1);
        }
    }
}
