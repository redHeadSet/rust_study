// 러스트에만 있는 개념이라고 한다. 아마 코드 상으로 직접 명시하는 건 처음이라 그런 듯
// 댕글링 참조자를 방지하기 위함
// - fail.rs 의 댕글링 참조자 예시 확인

// ================================================================
// lifetime 측정법
// 1. 함수의 각 인자는 고유한 lifetime을 가짐. 1개면 1개, 2개면 2개...
// 2. 1개의 인자만 있다면,출력 파라미터도 동일한 lifetime을 가짐
// 3. 여러 인자 중 하나가 self 라면, self의 lifetime이 모든 출력 파라미터에 대입
// ================================================================


fn main() {
    // lifetime 중의 예외, static lifetime
    // 다른 언어의 static 과 동일하게
    let s: &'static str = "static lifetime";
    lifetime();
}

// 아래 함수에서 <'l> 로 지정된 것이 라이프타임
fn longest<'l>(x: &'l str, y: &'l str) -> &'l str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// ================================================================
// 아래와 같은 구조체도 정의 부분에 lifetime 정의가 필요할 수 있다
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}