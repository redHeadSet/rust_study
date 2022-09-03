/*
전체적으로 코드가 연습하다보니 개떡같이 짜져있다
원래 소스와도 좀 다름
*/

use std::env;
use std::error::Error;
use std::process;

// 일반적인 rust 개발 방법론에 의거, lib.rs 파일로 대부분 이동
use greprs::my_greprs::*;

// 아래 read_file 에서 에러 발생할 여지가 있다.
// main() 함수에서 캐치해보도록 해보자
fn run(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let configs = parse_args(&args);
    read_file(configs)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args : {:?}\n", args);

    // run 함수 에러에 대한 처리
    if let Err(e) = run(args) {
        println!("Error catch... : {}", e);
        process::exit(1);
    }
}
