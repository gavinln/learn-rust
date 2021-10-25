fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let item = largest(&number_list);
    println!("The largest number is {}", item);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let item = largest_i32(&number_list);
    println!("The largest number is {}", item);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let item = largest_char(&char_list);
    println!("The largest char is {}", item);

    let item = largest_generic(&number_list);
    println!("The largest number is {}", item);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let item = largest_generic(&char_list);
    println!("The largest char is {}", item);

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer Point is {:?}", integer);
    println!("float Point is {:?}", float);

    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("integer Point2 is {:?}", both_integer);
    println!("float Point2 is {:?}", both_float);
    println!("integer_and_float Point2 is {:?}", integer_and_float);

    #[allow(dead_code)]
    enum Option2<T> {
        Some(T),
        None,
    }

    #[allow(dead_code)]
    enum Result2<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let point = Point { x: 3.0, y: 4.0 };
    println!("point_x is {}", point.x());
    println!("distance_from_origin is {}", point.distance_from_origin());

    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
