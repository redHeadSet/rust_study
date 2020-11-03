fn main() {
    println!("Hello, world!");
    //println!("{}",value_in_cents(Coin::Quarter(UsState::I)));
    println!("{}",value_in_cents(Coin::Nickel));
    //plus_one(Some(1));
    plus_one(None);


}
#[derive(Debug)]
enum UsState{
    M,
    I,
    N,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
    //use super::Coin;
    //use crate::Coin; //아앗.. 이미 여기에 있다..
    //match coin {
    //    Coin::Penny => 1,
    //    Coin::Nickel =>5,
    //    Coin::Dime =>10,
    //    Coin::Quarter(state) => {
    //        println!("뭐어어어{:?}", state);
    //        25
    //    },
    //}
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("Stateeee {:?}", state)
    } else {
        count += 1;
    }
    count
} 

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i+1),
        None => {
            println!("nooooone");
            None
        }
    }
}
