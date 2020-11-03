fn main() {
    use std::collections::HashMap;
    // use std::collections::hash_map; // 얜 모지?

    // HashMap<K, V>
    let mut scores = HashMap::new();

    // 하나씩 넣는 방법
    scores.insert(String::from("Red"), 10);
    let scores2 = String::from("Blue");
    let item = 50;
    scores.insert(scores2, item);

    // println!("{}", scores2); // scores2 는 copy 트레잇이 없어 소유권 뺏김
    println!("{}", item);

    // 여러 개 한 번에 지정
    let keys = vec![String::from("Yellow"), String::from("Black")   ];
    let item = vec![20,                     30                      ];

    let scores2: HashMap<_, _> = keys.iter().zip(item.iter()).collect();

    // for each_str in keys{
    //     println!("{}", each_str);   // 소유권 유지됨 but 아래쪽에서 scores2를 사용할 때 borrow 에러
    // }

    // === 값 접근 ===
    let search_key = String::from("Yellow");
    match scores2.get(&search_key){
        Some(i) => println!("Yellow item is {}", i),
        _ => println!("is None..."),
    }

    for (each_key, each_item) in scores2{
        println!("{} : {}", each_key, each_item);
    }

    // === 값 갱신 ===
    scores.insert(String::from("Red"), 100); // Red 값을 100으로 덮어쓰기
    scores.entry(String::from("Blue")).or_insert(200);  // Blue 값이 있다면 그대로 둠
    scores.entry(String::from("Black")).or_insert(200); // Black 값이 없다면 200 입력

    println!("{:?}", scores);

    // 각 요소에 대한 갱신
    let text = "hello world wonderful world";
    let mut hash_update = HashMap::new();
    
    for word in text.split_whitespace(){
        let each_count = hash_update.entry(word).or_insert(0);  // 각 요소가 나오면 해당 값 가져옴 or 없는 경우 0 으로 입력
        *each_count += 1;                                       // 가져온 값에 대해 +1 처리
    }
    println!("{:?}", hash_update);
}