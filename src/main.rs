use rand::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number? ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number: {secret_number}");

    println!("Pless input your guess");
    let mut guess = String::new(); //variable that stores user inputs
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess: u32 = guess.trim().parse().expect("please type a number");
    println!("you guessed {}", guess);

    loop {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => println!("you win"),
        }
    }
}

// let x = 5;
// let y = 10;
// println!("x: {x}, y: {}", y + 2);

// let mut sec = thread_rng();
// let blue = sec.gen_range(1..=5);
// println!("blue: {blue}");
