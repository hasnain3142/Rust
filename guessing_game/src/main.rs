#![allow(dead_code)]
use std::io;
use std::cmp::Ordering;
use::rand::Rng;

fn main() {
    println!("Welcome To The Guessing Game");

    let secret_number: i32 = rand::thread_rng().gen_range(0,100);
    println!("The secret number is {}",secret_number);
    loop{
        println!("Please Enter Your Guess Below:");
        let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("FAILED");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Congratulations!!!, You Win!");
                break; },
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Low!"),
        }
    }
}
