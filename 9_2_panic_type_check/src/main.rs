// 2 장의 랜덤 게임을 가져왔다
// 기존 코드의 문제로, Guess 값을 1~100 사이의 값이라면, 입력값도 그에 맞게 입력되어야 하는데,
// 해당 처리를 매 번 모든 코드에 넣는 것은 비효율적으로, struct 처리를 통해 간단히 처리가 가능하다

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use panic_type_check::guess::Guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match input.as_str().trim() {
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

        Guess::new(guess); // 에러 체크
        // 아마, 원래라면 Result 값을 이용하여 Err 처리를 진행해야 하는 게 맞을 거 같다

        println!("You guessed: {}", guess);

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