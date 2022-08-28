// trait : 특성
// 즉, 같은 특성을 가진 타입에 대해 동일한 동작을 추상화 할 수 있음
// 그런 동작을 정의하는 것을 트레잇 바운드(trait bound) - 인터페이스와 유사하지만 다른 점이 있음

// lib.rs 내 트레잇 정의
// ================================================================


// 아래는 10_1 에서 봤던 튜토리얼 코드를 트레잇과 제네릭을 이용하여 간단히 구현했다
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
