// RUST_BACKTRACE=1 cargo run
// RUST_BACKTRADE 키워드 사용 시, 콜스택이 보이게 됨

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn read_username_from_file()
        -> Result<String, io::Error> {  // return 값이 다음과 같은 경우
                                        // Result 값을 반환함. but 성공 시 String, 실패 시 io::Error 타입을 반환
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 위 read_username_from_file 함수와 거의 동일
// 다른 점은 ? 에서 에러 발생 시, From 트레잇 내의 from 함수를 호출
fn read_username_from_file_use_question_mark() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt") ? ;
    // let mut s = String::new();
    // f.read_to_string(&mut s) ? ;

    // 위 3줄과 동일
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s) ? ;
    Ok(s)
}
// ? 는 return이 있는 함수에서만 사용 가능하다. main 에서는 사용 불가

fn panic_test() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {  // kind 매치 가드
                                                                    // match 줄기 상으로 한 번 더 정제 가능
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("There was a problem opening the file: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

fn panic_test_unwrap() {
    let f = File::open("hello.txt").unwrap();                           // 시스템이 주는 메시지 출력
    let f = File::open("hello.txt").expect("hello open error expect");  // 내가 직접 설정하는 메시지 출력
}

fn main() {
    let ruff = read_username_from_file();
    let ruff = read_username_from_file_use_question_mark();

    panic_test();
}