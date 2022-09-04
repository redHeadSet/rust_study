use std::thread;
use std::time::Duration;

// T는 인자가 u32, 리턴이 u32 인 함수를 뜻한다
// Fn은 표준 라이브러리의 트레잇 - Fn, FnMut, FnOnce 트레잇도 있다
struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T:Fn(u32) -> u32 {
    fn new(calculation:T) -> Cacher<T>{
        Cacher {
            calculation,
            value: Option::None,
        }
    }

    fn value(&mut self, args:u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            }
        }
    }
}


// 함수로 정의
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;    // 운동 강도
    let simulated_random_number = 7;            // 랜덤 숫자

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 클로저 정의
    // 두 개 이상인 경우 컴마로 구분 |num1, num2|
    let expensive_closures = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cacher = Cacher::new(expensive_closures);

    if intensity < 25 {
        println!( "Today, do {} pushups!", cacher.value(random_number) );
        println!( "Next, do {} situps!", cacher.value(random_number) );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!( "Today, run for {} minutes!", cacher.value(random_number) );
        }
    }
}

/*
클로저는 함수와 다르게 파라미터나 리턴값을 명시하지 않음
    -> 함수는 사용하는 입장에서 파라미터와 리턴값, 이름 등등을
       모두가 알고 그에 합의하는 것을 보장해야 하기에 명시

클로저는 변수에 저장되고 이름없이 노출되지 않고 사용
    -> 보통 짧고 좁은 문맥 내에 관련있음
    -> 컴파일러가 안정적으로 파라미터 및 리턴 추론 가능

물론, 클로저도 타입 명시가 가능하긴 함 : |num: u32| -> u32
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

타입이 추론된 후, 다른 타입의 인자로 호출하는 것은 에러 발생
    ex. String 을 넣어 호출 후에 u32 를 넣는다던지...
*/