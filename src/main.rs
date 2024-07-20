// use rand::*;
// use std::cmp::Ordering;
// use std::io;

// fn main() {

//     println!("Guess the number? ");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     //println!("secret_number: {secret_number}");
//     loop {
//         println!("Pless input your guess");
//         let mut guess = String::new(); //variable that stores user inputs
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("failed to read line");
//         //ignore if the user inputs non number
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("you guessed {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("too small"),
//             Ordering::Greater => println!("too big"),
//             Ordering::Equal => {
//                 println!("you win");
//                 break;
//             }
//         }
//     }
// }

// age pass before playing aguessing game

// let x = 5;
// let y = 10;
// println!("x: {x}, y: {}", y + 2);

// let mut sec = thread_rng();
// let blue = sec.gen_range(1..=5);
// println!("blue: {blue}");

//tuples
// fn main() {
//     let tup: (i32, f64, u32) = (500, 56.4, 32);
//     println!("first: {}, second: {}, third: {}", tup.0, tup.1, tup.2);
// }

// fn main() {
//     let tup = (2, 3, 4.5);
//     let (x, y, z) = tup;
//     println!("x: {x}, y: {y}, z:{z} ")
// }

// use std::io;

// //
// fn main() {
//     let months: [&str; 13] = [
//         "o",
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//     let mut month = String::new();
//     println!("Enter a number from one to 12 to show you the month it represents");
//     io::stdin().read_line(&mut month).expect("Not a month");

//     let month: usize = month.trim().parse().expect("you didn't enter a number");
//     // let month = month + 1;
//     let element = months[month];
//     println!("The month with number {month} is {element} ");
// }

//struct
//calculate the area of a rectangle
// struct Rectangle {
//     height: u32,
//     width: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         height: 2,
//         width: 6,
//     };

//     println!("The area of the rectangle is:{} ", rect1.area());
// }

// vectors
fn main() {
    // let mut v: Vec<i32> = Vec::new();
    //updating a vector
    // v.push(5);
    // v.push(6);
    // v.push(12);
    // println!("This is v:, {v}");
    let v = vec![1, 2, 3, 4, 5];
    //refrence the third value via indexing
    let third: &i32 = &v[2];
    println!("the third value: {third}");

    //refernce the third value via get
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third value via the get method is {third}"),
        None => println!("no third element"),
    }
}
