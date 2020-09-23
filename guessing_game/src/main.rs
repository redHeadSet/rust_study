extern crate rand;

use std::io;
// use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    match guess.as_str().trim() {
        "K" => println!("ooo"),
        _   => println!("xxx")
    }

    loop {
        println!("Please input your guess. A : {}", secret_number);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(errmsg) => {
                println!("{}", errmsg);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // match guess >= secret_number {
        //     true  => println!("true "),
        //     false => println!("false")
        // }

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}