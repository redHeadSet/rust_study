/*
rust 반복자는 lazy
실제로 항목을 사용하고, 반복자를 소비하기 전까지는 아무 동작도 하지 않음
*/

fn main() {
    let v1 = vec![1,2,3,4];

    let _v1_iter = v1.iter();    // lazy 동작으로, 아무 처리도 하지 않음
}

// 실제 반복자는 Iterator trait을 가지고 next 메서드를 가짐
// std::iter::Iterator trait & method
/*
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; << Required method
    ...
}
*/
