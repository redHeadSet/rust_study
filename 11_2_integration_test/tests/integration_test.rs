// cargo test 시에, src와 동일한 depth 에서 tests 디렉토리를 찾아서 테스트한다
// 여기서 통합 테스트를 지원할 수 있음

extern crate integration_test;

// common 에 있는 모듈을 가져와서 사용 가능
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, integration_test::add_two(2));
}
