use std::io;

fn main() {
    let x = ['h', 'i', 'd', 'e', 'a','k','i'];
    let finish = ["Correct password !", "Incorrect password"];

    let mut guessing = String::new();

    io::stdin().read_line(&mut guessing).expect("Can't read line");

    let guessing: &str = guessing.trim();

    if !guessing.contains(&x[0..6]) {
        println!("{}", finish[1])
    } else {
        println!("{}", finish[0])
    }
}