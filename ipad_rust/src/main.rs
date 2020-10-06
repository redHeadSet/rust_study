use std::io;
fn main() {
    println!("Hello, world!");
    //fb_check();
    //cs_check();
    //ct_check();
    //fg_check();
}
fn fb_check() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("ERROR");
    println!("input number is : {}", number);
    let number:i32 = number.trim().parse().expect("ERRRRRROR");

    println!("FINAL fb : {}",fb(number));
}
fn fb(x: i32) -> i32 {
    if x == 0 || x == 1 {
        x
    }
    else {
        fb(x-1) + fb(x-2)
    }
}
fn cs_check() {
    //let Carol = "The Twelve Days of Christmas";
    let mut Carol = String::from("The Twelve Days of Christmas");
    let mut str_len = Carol.chars().count();
    let mut char_carol = Carol.chars();
/*
    let mut char_carol = Carol.chars();
    println!("String length is {}",str_len);
    while char_carol.next()!=None {
        println!("{}",char_carol.as_str());
    }
*/
    for c in char_carol {
        print!("{}",c);
    }
}
fn ct_check() {
    println!("Fahrenheit -> Celsius : 1, Celsius -> Fahrenheit : 2");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("ERROR");
    println!("input number is : {}", number);
    let number:i32 = number.trim().parse().expect("ERRRRRROR");

    println!("{} ---- Input Temperature ", number);
    let mut temper = String::new();
    io::stdin().read_line(&mut temper).expect("ERROR");
    println!("input temp is : {}", temper);
    let mut temper:f32 = temper.trim().parse().expect("ERRRRRROR");
    if number == 1 {
        temper = ((temper - 32.0 ) / 1.8);
    } else if number == 2 {
        temper = ((temper * 1.8) + 32.0);
    } else {
        println!("Plz check yiur input value. {}", number);
        return;
    };
    println!("{}", temper);

}
fn fg_check() {
    let r = ["1", "2", "3", "4", "five golden ring", "6", "7"];

    for (i, c) in r.iter().rev().enumerate() {
            //println!("{}{}", i, c);
        for count in (0 .. (i+1)).rev()
        {
            println!("{}",r[count]);
        }
    }
}