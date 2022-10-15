fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "{}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!(
        "{}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {}", area1(width1, height1));

    let rect1 = (30, 50);

    println!("The area of the rectangle is {}", area2(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:?}", rect2);

    println!("The area of the rectangle is {}", area3(&rect2));

    println!("The area of the rectangle is {}", rect2.area());

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Rectangle can hold other {}", rect3.can_hold(&rect2));

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Rectangle can hold other {}", rect4.can_hold(&rect3));

    println!("Square {:?}", Rectangle::square(20));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
