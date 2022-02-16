use std::io;
use std::{thread, time::Duration};

fn main() {
    let x = ['h', 'i', 'd', 'e', 'a','k','i'];
    let finish = ["Incorrect password !", "Correct password !"];
    let mut guessing = String::new();
    io::stdin().read_line(&mut guessing).expect("Can't read line");
    let guessing: &str = guessing.trim();
    let x: String = x.iter().collect();
    println!("{}", finish[(guessing.eq(&x[0..7]) as usize)]);
    thread::sleep(Duration::from_millis(2000))
}
