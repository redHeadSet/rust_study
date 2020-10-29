#[derive(Debug)]
enum IpAddrKind{
    V4(i32, i32, i32, i32),
    V6(String),
}

enum Message{
    Quit,
    Move{ x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Quarter(UsState),
}

//==================================================

impl IpAddrKind{
    fn print(&self) {
        println!("{:?}", self);
    }
    
    // Kind에 따라서 print 함수를 재정의...?
    fn print_each(&self) {
        match self{
            IpAddrKind::V4(v4_num_1, v4_num_2, v4_num_3, v4_num_4) => {
                println!("V4 items is {}.{}.{}.{}", v4_num_1, v4_num_2, v4_num_3, v4_num_4);
            },
            IpAddrKind::V6(each_v6) => {
                println!("V6 is {}", each_v6);
            }
        }
    }
}

//==================================================

fn main() {
    let ipv4 = IpAddrKind::V4(1,2,3,4);
    let ipv6 = IpAddrKind::V6(String::from("2001:0DB8:0000:0000:0000:0000:1428:57ab"));

    println!("ipv4 is...");
    ipv4.print();
    ipv4.print_each();
    println!("\nipv6 is...");
    ipv6.print();
    ipv6.print_each();

    let six = Option::Some(6);
    let good = Option::Some(String::from("good"));
    let null_int: Option<i32> = Option::None;

    let penny_coin = Coin::Penny;
    let alabama_coin = Coin::Quarter(UsState::Alabama);

    println!("penny value : {}", value_in_cent(penny_coin));
    println!("nickel value : {}", value_in_cent(Coin::Nickel));
    println!("alabama value : {}", value_in_cent(alabama_coin));

    let seven = plus_one_option(six);
    if let Option::Some(7) = seven{
        println!("is 7!!");
    } else {
        println!("not 7...");
    }
}

fn value_in_cent(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Quarter(stat) => {
            println!("this : {:?}", stat);
            4
        },
    }
}

fn plus_one_option(input: Option<i32>) -> Option<i32> {
    match input{
        Option::Some(i) => Option::Some(i+1),
        _ => Option::None,  // 나머지 모두
        // Option::None => Option::None,
    }
}