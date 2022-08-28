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
    // let mut s = String::new();
    // let mut f = File::open("hello.txt") ? ;
    // f.read_to_string(&mut s) ? ;

    // 위 3줄과 동일
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s) ? ;
    Ok(s)
}
// ? 는 전체 함수로부터 일찍 빠져나와 호출하는 코드에게 Err을 전달
// ? 는 return Result 인 함수에서만 사용 가능하다. main 에서는 사용 불가 -> 에러를 전파해야 하기 때문

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
    let f = File::open("hello.txt").unwrap();
    // 시스템이 주는 메시지 출력
    // unwrap 을 쓸 때는, 컴파일러가 모르는 정보를 내가 확실히 알고 있을 때 쓰는 것이 좋다
    // 간단히 말해, 절대 Err variant 를 발생시키지 않는 확신이 있다면 unwrap 을 사용하는 게 허용된다

    let f = File::open("hello.txt").expect("hello open error expect");
    // 내가 직접 설정하는 메시지 출력

}

// panic! 을 써야 할 때
// 1. 벌어질 일로 예상하지 않는 것
// 2. 그 이후의 코드가 해당 상태에 있으면 안 될 때
// 3. 이런 상태에 대해 처리할 만한 뾰족한 수가 없는 경우

fn main() {
    let ruff = read_username_from_file();
    let ruff = read_username_from_file_use_question_mark();

    panic_test();
}

// panic 을 호출할 때는 개발자 스스로 '복구 불가능한 에러' 라고 결정하는 것과 같다
// exam, test, prototype 인 경우에는 panic! 이 좀 더 일반적
//
// 반대로, Result 를 호출하는 경우는 옵셔널한 방법을 제공하는 느낌

