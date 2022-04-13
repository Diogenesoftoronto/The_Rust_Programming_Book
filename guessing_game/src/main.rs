use std::io;
use std::process;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("This is a guessing game.");
    println!("Are you ready?");
    let mut ready = String::new();
    io::stdin()
        .read_line(&mut ready)
        .expect("Failed to read line");
    if ready.trim() == "n" {
        eprintln!("Strike out! 🏏");
        process::exit(0x0100); // exit with error 256
    }
    let random_num: u8 = rand::thread_rng().gen_range(1..101);
    println!("Input a number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line. ");
            println!("You guessed {}", guess);
            let guess_int: u8 = guess.trim().parse().expect("Error, 🚩 Please input a number! 🚩");
            match guess_int.cmp(&random_num) {
                Ordering::Less => println!("too small 🍓"),
                Ordering::Greater => println!("too large 🍌"),
                Ordering::Equal => {println!("nice 6️⃣ 9️⃣ You win! 🌟"); break;},
        }
    }
 
}                                                                                                                                                                                       
