use std::io;
use std::{thread, time::Duration};

fn main() {

    let mut x = ['h', 'i', 'd', 'e', 'a','k','i'];
    let finish = ["Correct password !", "Incorrect password"];

    let mut guessing = String::new();

    io::stdin().read_line(&mut guessing).expect("Can't read line");

    let guessing: &str = guessing.trim();

    let x: String = x.iter().collect();

    if !guessing.contains(&x[0..6]) {
        println!("{}", finish[1]);
        thread::sleep(Duration::from_millis(4000));
    } else {
        println!("{}", finish[0]);
        thread::sleep(Duration::from_millis(4000));
    }
}