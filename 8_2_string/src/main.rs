fn main() {
    let mut str_a = String::new();  // 빈 스트링 값 생성

    let str_b_split = "abcde";
    let str_b = str_b_split.to_string();
    let str_b = "abced".to_string();    // 위, 아래 동일한 문장

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);   // 윗 줄에서 '참조'를 넘겼기 때문에 코드 문제 없음
    s1.push('1');
    println!("{}", s1);

    // let s1 = "abc";
    // let s2 = "def";
    // let s3 = s1 + &s2;   // 단순 스트링 리터럴로는 작업 불가

    let s1 = String::from("abc");
    let s2 = String::from("def");
    let s3 = String::from("ghi");
    let s4 = s1 + "_" + &s2 + "_" + &s3; // 조합 가능
    // println!("{}", s1); // 첫 인자 s1의 경우, 소유권이 s4로 넘어가서 사용 불가
    println!("{}", s3);

    let s5 = format!("{} - {}", s2, s3); // format 방식으로 사용하는 경우, 위와 달리 s2, s3의 소유권이 유지됨
    println!("{} , {}+{}", s5, s2, s3);

    let s1 = "abcedfghij";
    // let index_s1 = s1[0];   // 거부. 러스트는 인덱싱을 지원하지 않음

    // String 은 Vec<u8> 을 감싼 것.
    let length_s1 = s1.len();
    let length_s2 = "가나다라마".len();
    let length_s3 = "☆★○※".len();
    let length_s4 = "Здравствуйте".len();
    println!("{} vs {} vs {} vs {}", length_s1, length_s2, length_s3, length_s4);
    // UTF-8 방식의 여러 문자가 글자 구현 방식에 대해 다르기에 인덱싱을 지원하지 않음
    // ex. 영문인 경우 1byte, 한글은 2byte 등... 근데 왜 한글 3byte 같이 처리되지...?

    let die_string_slice = "Здравствуйте";
    let is_ok = &die_string_slice[0..4];
    println!("is ok : {}", is_ok);
    // let is_not_ok = &die_string_slice[0..1];
    // println!("is not ok... {}", is_not_ok);  // 런타임 사망... 유효하지 않은 인덱스 접근하면 안됨 ㅠㅠ

    // 대신에 이렇게 사용 가능
    for each_char in die_string_slice.chars() {
        print!("{} + ", each_char);
    }

    println!("");

    for each_byte in die_string_slice.bytes() {
        print!("{} + ", each_byte);
    }

    
}