use std::io;

fn main() {
    println!("Write the First Word :");

    let mut word = String::new();
    //let firstword = String::from("first");
    //let mut secondword = String::new();
    //let secondword = String::from("apple");

    io::stdin().read_line(&mut word).expect("Failed to read line");

    let firststr = &word[0..1];

    // a, e, i, o, u 일때 체크해서
    // 첫번째 함수, 두번째 함수 들가게 하고싶음
    match  firststr {
         "a"| "e"| "i"| "o"| "u" => secondfn(&word),
        _ => firstfn(&word),
    }
}

fn firstfn(s: &String ){
    let length = s.len();

    let first = &s[0..1];
    let strtemp = &s[1..length];

    let changefirst = format!("{}{}", first, String::from("ay"));

    println!("First Word : {}-{}", strtemp.trim(), changefirst);
}


fn secondfn(s: &String ){
    println!("Second Word : {}-hay", &s.trim());
}