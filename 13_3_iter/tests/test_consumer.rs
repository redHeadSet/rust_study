#[test]
fn iter_consumer() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();

    assert_eq!(sum, 6);
}
// sum 같은 경우, 내부적으로 next 함수를 호출하여 반복자를 소비한다
// next 함수를 사용하면서 반복자를 소비하는 함수 : '어댑터'라고 칭함

/*
#[test]
fn iter_map_consumer_warning() {
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);
}
*/
// 단순 반복자와 map 호출하는 위 소스의 경우에는 반복자는 lazy하므로 실제로 아무런 동작도 하지 않는다
// 아래와 같이 collect 함수를 쓰는 것으로 개선

#[test]
fn iter_map_consumer_collect() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
// collect 를 호출함으로서 결과를 컬렉션 형태로 return

