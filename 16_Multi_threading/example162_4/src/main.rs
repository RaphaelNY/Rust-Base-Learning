use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // error: value used here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
