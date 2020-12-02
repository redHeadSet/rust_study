// 스트링을 피그 라틴(pig Latin)으로 변경해보세요.
// 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 “ay”를 붙이므로, “first”는 “irst-fay”가 됩니다.
// 모음으로 시작하는 단어는 대신 끝에 “hay”를 붙입니다. (“apple”은 “apple-hay”가 됩니다.)
// UTF-8 인코딩에 대해 기억하세요!

use std::io;

fn main() {
    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string)
    .expect("Failed to read line");

    let first_some = input_string.trim().chars().next();
    let first_char = match first_some{
        Some(c) => c,
        None => {
            println!("error...");
            return
        },
    };

    let output_string = match first_char{
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => hay_string(input_string),
        _ => any_ay_string(input_string, first_char),
    };

    println!("outs : {}", output_string);
}

fn hay_string(input_string:String) -> String{
    let output_string = format!("{}-hay", input_string.trim());
    output_string
}

fn any_ay_string(input_string:String, first_char:char) -> String{
    let mut forward_string = String::new();
    for (i, each_char) in input_string.trim().chars().enumerate(){
        if i == 0 { continue; }
        forward_string.push(each_char);
    }

    let output_string = format!("{}-{}ay", forward_string, first_char);
    output_string
}