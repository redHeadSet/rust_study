// cargo new --iib module
// lib 파일은 cargo build (run 할 것이 없으므로)

// 러스트는 기본적으로 lib.rs 파일만 찾아볼 줄 안다!
// 나머지 모듈은 아래쪽에 선언하는 식으로 작성해야 함.

/*
 * 모듈 파일 시스템의 규칙
 * 파일에 관한 모듈의 규칙을 정리해봅시다:
 *
 * 만일 foo라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, foo.rs라는 이름의 파일 내에 foo에 대한 선언을 집어넣어야 합니다.
 * 만일 foo가 서브모듈을 가지고 있다면, foo/mod.rs라는 이름의 파일에 foo에 대한 선언을 집어넣어야 합니다.
 * 이 규칙들은 재귀적으로 적용되므로, foo라는 이름의 모듈이 bar라는 이름의 서브모듈을 갖고 있고 `bar는 서브모듈이 없다면, 여러분의 src 디렉토리 안에는 아래와 같은 파일들이 있어야 합니다:
 */

pub mod client;
// 타 파일에서 선언하도록
// {
//      contents of client.rs...    // -> 그래서 client.rs 파일에서는 mod 선언 및 {} 가 없음.
                                    // cf. 또 mod 선언을 하게 되면 하위 mod 를 만들게 됨!
// }

// 2. pub 선언의 경우, 외부에 client 모듈을 public 으로 선언하여 접근할 수 있게 함

pub mod network;    // 얘는 network.rs 파일을 못 찾는 경우, newwork 디렉토리 밑 mod.rs 파일을 찾는 것으로 보임



// public 확인 TEST ================================
// mod outermost {
//     pub fn middle_function() {}
//     fn middle_secret_function() {}
//     mod inside {
//         pub fn inner_function() {}
//         fn secret_function() {}
//     }
// }

// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
// }
// public 확인 TEST ================================

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // super::network::connect();  // 부모 영역 접근 시 super 사용!
        network::connect(); // 위 이름 영역에 super 선언으로 가능
    }
}
