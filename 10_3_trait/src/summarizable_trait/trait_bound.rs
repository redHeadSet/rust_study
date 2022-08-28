use crate::summarizable_trait::Summarizable;
use core::fmt::Debug;

// trait bound 구현
// T 제네릭은 Summarizable 트레잇을 가지고 있어야 한다
// 즉, notify 함수는 Summarizable 값을 가진 인자만 받을 수 있도록 제한
pub fn notify<T: Summarizable>(item: &T) {
    println!("noti!! {}", item.summary());
}

// 트레잇을 2개 이상 쓰고 싶은 경우에는
pub fn two_trait_noti<
    T1: Summarizable + Clone,
    T2: Clone + Debug
>(item1: T1, item2: T2) -> T2 {
    println!("two trait noti {}", item1.summary());
    item2.clone()
}

// 위 함수는 너무 길고 가독성도 떨어짐
// 아래와 같이 변경
pub fn two_trait_noti_short<T1, T2>(item1: T1, item2: T2) -> T2
    where T1: Summarizable + Clone,
          T2: Clone + Debug,
{
    println!("two trait noti {}", item1.summary());
    item2.clone()
}