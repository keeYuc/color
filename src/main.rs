use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

fn handle(str: String) -> String {
    str
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("1");
        match tx.send(buf) {
            Ok(_) => {}
            Err(_) => {
                println!("err");
                break;
            }
        }
    });
    for item in rx.recv() {
        println!("{}", handle(item));
    }
}
