use std::io;

fn main() {
    println!("Guess the number? Pless input your guess");
    let mut guess = String::new(); //variable that stores user inputs
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed {}", guess);
}

// let x = 5;
// let y = 10;
// println!("x: {x}, y: {}", y + 2);
