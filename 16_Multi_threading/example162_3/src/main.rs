use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("hello");
        tx1.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let received1 = rx.recv().unwrap();
    println!("Got: {}", received1);
}
