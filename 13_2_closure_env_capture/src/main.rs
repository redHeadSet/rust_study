fn main() {
    cls_env_fn();
    cls_env_fn_once();
}

/*
3가지 Fn 트레잇
1. FnOnce : 클로저를 둘러싼 환경 변수를 소비 (소유권 가짐) -> move 키워드 사용
2. Fn : 환경 변수를 불변으로 빌려옴
3. FnMut : 환경 변수를 가변으로 빌려옴 - 환경 변경 시 변경됨
*/

fn cls_env_fn() {
    let x = 3;

    // 클로저는 호출되었을 당시의 환경을 캡쳐할 수 있다
    // 클로저 바디에서 그 값을 저장하기 위해 메모리를 사용한다 : 오버헤드 발생 (함수와는 다름)
    let cls_env = |args| {
        args == x
    };
    // 이 부분을 함수로 선언한 경우 당연히 에러가 발생한다
    // fn func_env(args:u32) -> bool { args == x }

    let y = 4;
    cls_env(y);
}

fn cls_env_fn_once() {
    let x = vec![1,2,3];

    let cls_env = move |args| {
        args == x
    };

    // println!("x value is {:?}", x);
    // 위처럼 x에 대한 처리를 진행하려고 하면 소유권이 이동되었다고 에러가 발생

    let y = vec![4,5,6];
    cls_env(y);
}
