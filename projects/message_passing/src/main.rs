use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // rust standard library provides channels
    // a channel is a a programming concept by which
    // data is sent form one thread to another
    // a channel has two halves: transmitter & receiver
    // a channel is closed if either the transmitter or receiver is droppee
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx2, rx2) = mpsc::channel();

    let tx3 = tx2.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = String::from("more");
        tx3.send(val).unwrap();
    });

    for received in rx2 {
        println!("Got: {}", received);
    }
}
