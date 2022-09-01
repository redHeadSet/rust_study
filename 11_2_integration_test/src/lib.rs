pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// cargo new --lib 으로 생성했을 때, #[cfg(test)] 어노테이션이 추가된다
// 해당 어노테이션은 cargo build 에서는 제외되며, cargo test 를 실행할 때만 컴파일 & 실행된다
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2)); // 테스트는 비공개 함수도 접근이 가능
    }
}

/*
만약 우리의 프로젝트가 src/lib.rs 파일이 없고 src/main.rs 파일만 갖고 있는 바이너리 프로젝트라면,
tests 디렉토리 내에 통합 테스트를 만들어서 src/main.rs에 정의된 함수를 가져오기 위하여 extern crate를 이용할 수 없습니다.
오직 라이브러리 크레이트만 다른 크레이트에서 호출하고 사용할 수 있는 함수들을 노출시킵니다; 바이너리 크레이트는 그 스스로 실행될 것으로 여겨집니다.

이는 바이너리를 제공하는 러스트 프로젝트들이 src/lib.rs에 위치한 로직을 호출하는 간단한 형태의 src/main.rs를 가지고 있는 이유 중 하나입니다.
이러한 구조와 함께라면, extern crate를 이용하여 중요한 기능들을 커버하도록 하기 위해 통합 테스트가 라이브러리 크레이트를 테스트할 수 있습니다.
만일 중요 기능이 작동한다면, src/main.rs 내의 소량의 코드 또한 동작할 것이고, 이 소량의 코드는 테스트할 필요가 없습니다.
*/

// 위 글을 추론하자면, 일반적으로 lib.rs 를 호출하는 방식으로 코딩을 많이 하는 거 같다.
// main.rs 쪽은 많은 데이터가 들어가진 않는 듯