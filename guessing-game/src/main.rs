use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Try guessing the number from 0 to 10!");
    println!("Please enter your guess : ");

    let secret_num = rand::thread_rng().gen_range(1..=10);

    // let trial = "must";

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("You guessed : {guess}");
    println!("The secret number is {secret_num}");

    // if secretNum < 90 {
    //     println!("Mistki!");
    // }
    // if guess.trim() == "78"{
    //     println!("lilith");
    // }

    match guess.cmp(&secret_num){
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too big."),
        Ordering::Equal => println!("You guessed right!"),
    }
}
