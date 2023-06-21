/*
Source code for a guessing game Nim!

*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Nim!");
    
    let min = 1;
    let max = 100;
    let winning_number = rand::thread_rng().gen_range(min..=max);
    
    println!("The winning number is: {winning_number}");
    
    loop {
        
        println!("Take a guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //need to shadow it in order to compile since we are inputing a string
        //need to parse it and expect
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&winning_number)
        {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("Winner!!!!!!!");
                break;
            }
        }
    }
}