use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    //*****  9.2  *****//
    // let f = File::open("hello.txt");
    // let f = match f {
        //  Ok(file) => file,
        //  Err(error) => {
        //    panic!("There was a problem opening the file: {:?}", error)
        //  },
    // };

    //에러 여러개 매칭하기=======
    /*
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {//매치가드 - 패턴에 ref가 필요함
            match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "파일 생성에 실패했어요!!! {:?}", e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "파일 여는데 실패했어요!!!! {:?}", error
            )
        },
    };
    */
    //패닉을 위한 숏컷=========
    //let f = File::open("hello.txt").unwrap(); //자동으로 panic 매크로
    // 파일이 없으면  'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
    //let f = File::open("hello.txt").expect("내가 원하는 panic message..!");

    //에러 전파하기 ==========
    //에러를 처리하는 함수를 만들어보자
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
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
    let result1 = read_username_from_file();
    println!("{:?}", result1);
    */
    //에러 전파를 위한 숏컷 ?
    //와 대박ㅋㅋ
    //물음표 연산자를 사용할 때 에러값들이 표준 라이브러리 내에 있는 From 트레잇에 정의된 from 함수 침
    // 보일러플레이트 : 상용구 코드 - 어떤일을 하기위해서 꼭 작성해야 하는 코드 https://ko.wikipedia.org/wiki/%EC%83%81%EC%9A%A9%EA%B5%AC_%EC%BD%94%EB%93%9C
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?; //???????
        let mut s = String::new();
        f.read_to_string(&mut s)?; //???????
        Ok(s)
    }
    */
    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?; //???????
        Ok(s)
    }
    let result1 = read_username_from_file();
    println!("{:?}", result1);
    */
    //?는 Result를 반환하는 함수에서만 사용될 수 있습니다
    //let f = File::open("hello.txt")?; // 불가능
    
}
