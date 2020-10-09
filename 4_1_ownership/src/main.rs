fn main() {
    // Stack 스택 예제 ================================================
    let str_stack_1 = "stack!";
    let str_stack_2 = str_stack_1;
    println!("{} !!", str_stack_1);
    println!("{} !!", str_stack_2);
    // 스택 영역의 경우에는 값의 복사가 이루어진다.
    // 아래 힙 영역의 데이터와는 다르다.



    // heap 힙 예제 ===================================================
    let str_move_1 = String::from("hello"); // 힙 영역 데이터 선언
    let str_move_2 = str_move_1;
    println!("{} !!", str_move_2); // 가능!
    // println!("{} !!", str_move_1); // 불가능!
    // 유효하지 않은 참조자 사용 (str_move_1 은 move 되었다.)
    // + 러스트는 절대 깊은 복사를 하지 않는다 => 절대적으로 효율적이다.
    // 해당 내용, 즉 move 는 "힙 영역의 데이터"에서만 유효하다.



    // clone 예제 =====================================================
    let str_clone_1 = String::from("Clone Test!");
    let str_clone_2 = str_clone_1.clone();
    println!("{} !!", str_clone_1); // 가능!
    println!("{} !!", str_clone_2); // 가능!
    // clone() 의 경우 깊은 복사를 실행한다.



    // drop 예제 ======================================================
    {
        let test_drop = String::from("drops?");
        println!("{} !!", test_drop);
    }
    // println!("{} !!", test_drop); // 변수가 유효하지 않음.
    // test_drop 변수의 경우, 스코프 밖으로 벗어날 때 자동적으로 drop 함수를 호출하여 메모리 제거됨.
    // [주 의] : C 언어와 같은 new 이후에 delete 이전까지 힙 영역에 계속하여 살아있는 데이터와는 다름.



    // trait 트레잇 간단 설명 ===========================================
    // 스택에 저장하는 타입에는 Copy 트레잇이라는 특별한 어노테이션(주석)이 존재함.
    // ex) 정수형 등
    // 이와 같이 Copy 트레잇이 있는 경우에는 예전 변수 계속 사용 가능 (깊은 복사가 됨)
    // 반대로, drop 트레잇이 어노테이션되어 있는 타입이라면 Copy 트레잇을 어노테이션 할 수 없음.
    // Copy 트레잇 간단 목록:
    let copy_trait_1:u32 = 10;
    let copy_trait_2:bool = true;
    let copy_trait_3:f64 = 11.111;
    let copy_trait_4:(i32, f64) = (100, 3.14);
    println!("{}{}{}{}{}", copy_trait_1, copy_trait_2, copy_trait_3, copy_trait_4.0, copy_trait_4.1);

    // ===============================================================
}