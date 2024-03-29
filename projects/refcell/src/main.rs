use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // Interior mutability using RefCell
    // RefCell<T> allows mutable sharing with interior mutability
    // the patter used unsafe code to mutate data
    // References and Box<T> enforce invariants at compile time
    // RefCell<T> enforce invariants at run time
    // RefCell<T> and R<T> are for use in single-threaded programs only

    // Rc<T> allows multiple owners of the same data
    // Box<T> and RefCell<T> have single owners

    //Box<T> allows immutable or mutable borrows checked at compile time
    // Rc<T> only allows immutable borrows checked at compile time
    // RefCell<T allows immutable or mutable borrws checked at runtime

    // Combine Rc and RefCell to have multiple ownership of mutable dataj
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a: RefCell<Vec<i32>> = RefCell::new(vec![1, 2]);
    println!("a when created {:?}", a);
    let mut one_borrow = a.borrow_mut();
    let mut two_borrow = a.borrow_mut(); // panics as cannot have a second borrow_mut
    println!("{:?}", a);
    one_borrow.push(3);
    println!("{:?}", a);
    two_borrow.push(4);
    println!("{:?}", a);
}

trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(dead_code)]
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
