fn main() {
    let s = "Hello, world!";

    let word = first_word(&s[..]);
    println!("the first word is {}", word);
    
    let word = first_word(&s);
    // s.clear(); -> Error 빌림 규칙에서 불변 참조자를 만들었을 경우, 가변 참조자를 만들 수 없다.
    println!("the first word is {}", word);
    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();  // 문자열을 바이트 배열로 변환

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]   // 전체 문자열
}
