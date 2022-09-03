use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

pub mod config_structure;
use config_structure::GrepConfigs;

pub fn parse_args(args:&[String]) -> GrepConfigs {

    // unwrap 하며 Result 값을 받을 수 있지만
    // 에러 발생 시에는 |err| 클로저를 사용하여 에러 핸들링 가능
    GrepConfigs::parse_args(args).unwrap_or_else(|err| {
        println!("parse_args error : {}", err);
        process::exit(1);
    })
}

// std::error::Error 에 정의된 트레잇
// https://doc.rust-lang.org/std/error/trait.Error.html
// dyn 은 dynamic 의 약자 - 다양한 에러 상황에 다른 타입의 오류 반환 가능
pub fn read_file(configs: GrepConfigs) -> Result<(), Box<dyn Error>> {
    let _query = configs.get_query();
    let filename = configs.get_filename();

    let mut f = match File::open(filename){
        Ok(file) => file,
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
            panic!("file not found")
        },
        Err(error) => {
            panic!("some error {:?}", error)
        },
    };

    let mut contents = String::from("");

    // 아래 read_to_string 에서 ? 처리를 함으로서 expect 처리가 제외됨
    // 해당 에러는 다른 곳에서 catch 처리를 해줘야 하는 것으로 warning 발생
    // -> this `Result` may be an `Err` variant, which should be handled
    f.read_to_string(&mut contents)?;
    // 위 에러는 main()에서 잡아보자...

    println!("contents : {}", contents);

    Ok(())
}