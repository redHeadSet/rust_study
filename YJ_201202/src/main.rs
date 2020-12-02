// 201202 섭씨 화씨 상호변환 
use std::io;

fn main() {
    loop
    {
        let mut select = String::new();

        println!("[1] 섭씨->화씨");
        println!("[2] 화씨->섭씨");
        println!("[3] 종료");
        println!(">>");

        io::stdin().read_line(&mut select)
            .expect("Failed to read line");
        
        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                print!("{}\n", err);
                continue;
            },
        };

        if select == 3
        {
            println!("프로그램을 종료합니다.");
            break;
        }
        else if select == 1 || select == 2
        {
            let mut temperature = String::new();
            println!("변환할 온도를 입력하세요>>");
            io::stdin().read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("{}", err);
                    continue;
                },
            };

            // 섭씨->화씨
            if select == 1
            {
                let calc :f32 = temperature * 1.8 + 32.0;
                println!("섭씨{}->화씨{}", temperature, calc);
            }
            // 화씨->섭씨
            else if select == 2
            {
                let calc :f32 = (temperature - 32.0) / 1.8;
                println!("화씨{}->섭씨{}", temperature, calc);
            }
        }
        else{
            println!("보기에 없는 숫자입니다. 다시 입력하세요.");
        }
    }
}
