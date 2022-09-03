// 인자값을 읽어들이기 위해 표준 라이브러리 이용
use std::env;

// 파일 처리를 위해 아래 2개의 표준 라이브러리 추가
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // args collect 를 사용하여 인자를 받아올 수 있다
    // args 변수 타입을 대부분은 설정하지 않아도 추론되지만,
    // 현재와 같이 외부에서 받아오는 경우에는 추론하기 어렵기 때문에 <String>과 같이 명시
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // cargo run nee kee ann
    // -> ["target/debug/greprs", "nee", "kee", "ann"]
    // 첫 인자는 현재 바이너리 경로로 들어간다

    let query = &args[1];
    let filename = &args[2];

    // std::fs::File (struct)
    // open 의 return 은 Result<File>
    // https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    let mut f = match File::open(filename){
        Ok(file) => file,
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
            panic!("file not found")
        },
        Err(error) => {
            panic!("some error {:?}", error)
        },
    };
    // let mut f = File::open(filename).expect("file not found");


    let mut contents = String::from("");

    // File 의 read_to_string 함수는 std::io::Read 트레잇에 정의
    // return 은 Result<usize> 로 읽어들인 바이트 수를 반환
    // https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
    f.read_to_string(&mut contents).expect("file read fail");

    println!("contents : {}", contents);
}