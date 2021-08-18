use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        tx.send(buf).unwrap();
    });
    for item in rx.recv() {
        println!("{}", item)
    }
}

