use std::io;
// use rand::prelude::*;
use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("random number is {}", random_num);
    println!("Guess a number!");
    println!("Input a number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line. ");
    println!("You guessed {}", guess);
    
    if random_num == guess.trim().parse().unwrap() {
        println!("you guess right!")
    }
}
