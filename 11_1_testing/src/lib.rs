// 일반적으로 테스트는 아래 순서를 따름
// 1. 필요 데이터 혹은 상태 설정
// 2. 테스트 실행
// 3. 결과 비교

// test 관련 실행법은 sh 파일에 각자 저장

pub mod rectangle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rectangle::*;

    #[test] // 이 어노테이션으로 어떤 함수가 테스트로 다뤄야 하는지 확인 가능
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);  // 동일 비교
        assert_ne!(result, 5);  // 비동일 비교
    }

    #[test]
    #[should_panic] // 에러가 발생해야 정상인 테스트의 경우, 해당 어노테이션 추가
    fn fail_test() {
        panic!("error...!");
    }

    #[test]
    fn rect_test() {
        let large = Rectangle{ length: 8, width: 7 };
        let small = Rectangle{ length: 7, width: 6 };
        assert!(large.can_hold(&small));    // 성공 case
        assert!(!small.can_hold(&large));   // 실패 case
    }

    #[test]
    #[should_panic]
    fn fail_test_custom_comment() {
        let result: Result<String, i32> = Ok("ABC".to_string());
        assert!(
            result.as_ref().unwrap().contains("abc"), // 아래줄 없이 이것만 적으면 에러 줄 정보만 출력되는 수준
            "Result expect 'abc' but contain '{}'", result.as_ref().unwrap()    // 해당 줄에 정보를 추가함으로서 디테일한 에러 확인 가능
        );
    }

    #[test]
    fn test_output() {
        // cargo run -- --nocapture 옵션 사용 시 아래 글 보임
        println!("run nocapture output...!");
    }

    #[test]
    #[ignore]   // 해당 어노테이션이 있는 경우, 테스트에서 제외됨
    fn ignore_test() {
        panic!("ignore_test!!");
    }
}

// cargo test
// 해당 명령으로 test annotation 함수를 전체 실행한다