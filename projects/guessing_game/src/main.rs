use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number:i32 = rand::thread_rng().gen_range(-30..=30);
    //println!("The secret number is {secret_number}");
    
    loop {
        println!("Please input your guess");
    
        // Variables in Rust are immutable by default.
        // To make them mutable we will have to add "mut" keyword while declaring the variable
        // ::new means new is function associated with String type
        // references are also immutable by default. We will have to add mut keyword in order to make them mutable
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //The below statement will crash the program if the parse function results in error
        //let guess: u32 = guess.trim().parse().expect("Please type in a number"); 
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number)   {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("You won"); break;},
            Ordering::Greater => println!("Too big"),
        }
    }
}
