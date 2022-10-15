fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?} and {:?}", four, six);

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?} and {:?}", home, loopback);

    #[derive(Debug)]
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("{:?} and {:?}", home, loopback);

    #[derive(Debug)]
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    println!("{:?} and {:?}", home, loopback);

    #[derive(Debug)]
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("enum Message is {:?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    #[derive(Debug)]
    enum Option2<T> {
        None2,
        Some2(T),
    }

    let some_number = Option2::Some2(5);
    let some_string = Option2::Some2("a string");
    let absent_number: Option2<i32> = Option2::None2;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
}
