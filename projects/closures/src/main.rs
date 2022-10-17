use core::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            };
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1);
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("User with preference {:?} gets {:?}", user_pref2, giveaway2);

    // example closure
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("expensive_closure output {}", expensive_closure(3));

    // functions vs closures
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!("add_one_v1: {}", add_one_v1(2));
    println!("add_one_v2: {}", add_one_v2(2));
    println!("add_one_v3: {}", add_one_v3(2));
    println!("add_one_v4: {}", add_one_v4(2));

    // cannot use different types for the same closure: mismatched types
    // println!("add_one_v4: {}", add_one_v4(2.3));

    // capturing references or moving ownership
    immutable_borrowing();
    mutable_borrowing();

    // moving data to a closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // FnOnce - closures that can be called once
    // closures that move captured values out of its body

    // FnMut - closures that can be called more than once
    // closures that don't move captured values out of its body but
    // might mutate the capture values

    // Fn - closures that can be called more than once, possibly concurrently
    // closures that don't move captured values out of its body and
    // don't mutate captured values or don't capture outside values

    closure_fn_mut();
    closure_fn_once();
    closure_fn_mut_no_move_captured_value();
}

fn immutable_borrowing() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrowing() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_rectangle_list() -> [Rectangle; 3] {
    [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ]
}

fn closure_fn_mut() {
    let mut list = get_rectangle_list();
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn closure_fn_once() {
    let mut list = get_rectangle_list();
    // let mut sort_operations: Vec<&str> = vec![];
    // let value = String::from("by key called");

    list.sort_by_key(|r| {
        // cannot move captured value out
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}

fn closure_fn_mut_no_move_captured_value() {
    let mut list = get_rectangle_list();
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);
}
