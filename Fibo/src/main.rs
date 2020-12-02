// n번째 피보나치 수열 생성

use std::io;
fn main() {
    loop
    {
        let mut fibo = String::new();

        println!("몇번째 피보나치 수열의 값을 알고싶으세요??(종료는 0)>>");
        io::stdin().read_line(&mut fibo).expect("Failed to read line");

        let fibo :u32 = match fibo.trim().parse(){
            Ok(num) => num,
            Err(err) => {
                print!("{}\n", err);
                continue;
            },
        };
        let mut first:u64 = 1;
        let mut second:u64 = 1;

        if fibo <= 0
        {
            println!("종료합니다.");
            break;
        }
        else if fibo == 1 || fibo == 2
        {
            println!("{}번째 피보나치 값은 {}입니다.", fibo, 1);
        }
        else
        {            
            for i in (2..fibo)
            {
                let tmp = first + second;
                first = second;
                second = tmp;
            }
            println!("{}번째 피보나치 값은 {}입니다.", fibo, second);
        }
    }
}
