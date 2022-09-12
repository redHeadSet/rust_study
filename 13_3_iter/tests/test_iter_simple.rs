#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
// 위 코드처럼 next 함수를 호출하면서 반복자의 내부 상태가 변경됨 : mut 으로 선언된 이유...
// 이는 다른 말로, 반복자를 '소비' 한다고 함
// next 호출로 얻은 값은 벡터 내 값들은 불변 참조

#[test]
fn iter_ownership() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}
// 각 값에 대한 소유권을 가지고 싶다면, 위와 같이 into_iter() 로 반복자 호출
// 가변 참조에 대해서는 iter_mut() 호출