use custom_iter::custom_iter::Counter;

#[test]
fn counter_test() {
    let mut counter = Counter::new();

    let mut vector: Vec<Option<u32>> = Vec::new();

    for _i in 0..7 {
        vector.push(counter.next());
    }

    assert_eq!(vector.get(0).unwrap(), &Some(1));
    assert_eq!(vector.get(1).unwrap(), &Some(2));
    assert_eq!(vector.get(2).unwrap(), &Some(3));
    assert_eq!(vector.get(3).unwrap(), &Some(4));
    assert_eq!(vector.get(4).unwrap(), &Some(5));
    assert_eq!(vector.get(5).unwrap(), &Some(6));
    assert_eq!(vector.get(6).unwrap(), &None);

    // 이 코드는 왜 안되는지 확인 필요
    // for i in 0..7 {
    //     let iter_num: u32 = i.clone();
    //     let compare = match iter_num {
    //         0..=5 => Some(iter_num+1),
    //         _ => None,
    //     };
    //
    //     assert_eq!(vector.get(iter_num).unwrap(), &compare);
    // }
}