fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&chars);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}

// ================================================================
// 맨 위 2개의 함수의 시그니처는 아래와 같다
// fn largest<T>(list:&[T]) -> T
// 하지만 위와 같은 형태로 컴파일 시에 std::cmp::PartialOrd 트레잇 관련 에러 발생.
// 즉, T는 모든 상태에서 동작이 불가능하고 PartialOrd 트레잇이 있는 경우에만 가능하다
// 간단히 말해, 비교가 가능한 T에 대해서만 동작이 가능하다

// ================================================================
// 제네릭을 사용하는 대표적인 2 예가 다음과 같다
enum Option<T> {
    Some(T),
    None,
}
enum Result<T1, T2> {
    OK(T1),
    Err(T2),
}

// ================================================================
// 다음과 같이 혼용하여 사용 가능하다
struct Point<T1, T2> {
    x:T1,
    y:T2,
}

impl<T1, T2> Point<T1, T2> {
    fn mixup<V1, V2>(self, other:Point<V1, V2>) -> Point<T1, V2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// ================================================================
// 다른 언어처럼 제네릭을 사용하면서 런타임 비용이 들지 않는다
// 단형성화(monomorphization)를 수행함으로서 성능을 이끌어낸다
// 간단히 말해, 제네릭이 호출되는 곳을 확인하여 구체적인 타입에 대한 코드를 생성하여 처리한다
