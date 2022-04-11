use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random_num: u32 = rand::thread_rng().gen_range(1..101);
    println!("random number is {}", random_num);
    println!("Guess a number!");
    println!("Input a number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line. ");
    println!("You guessed {}", guess);
    let guess_int: u32 = guess.trim().parse().expect("Error, 🚩 Please input a number! 🚩");
    match guess_int.cmp(&random_num) {
        Ordering::Less => println!("too small 🍓"),
        Ordering::Greater => println!("too large 🍌"),
        Ordering::Equal => println!("nice 6️⃣ 9️⃣"),
    }
    // if random_num == guess.trim().parse().unwrap() {
    //     println!("you guess right!")
    // }
}
