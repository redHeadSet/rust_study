fn main() {
    // 복합 데이터 테스트 1
    let _set : (i32, i64, f32, String);
    _set = ( 32, 54, 1.2332, "ABCD".to_string() );

    println!("i32 : {}", _set.0 );
    println!("i64 : {}", _set.1 );
    println!("f32 : {}", _set.2 );
    println!("String : {}", _set.3 );

    // 복합 데이터 테스트 2
    let _data2 = (500, 6.4, "ABCD");
    let (x, y, z) = _data2;
    println!("x : {}", x);
    println!("y : {}", y);
    println!("z : {}", z);

    // 제어 테스트
    let condition = true;
    let if_num : i32 = if condition { 5 }
                       else { 6 };
    println!("if_num : {}", if_num);

    // 반복 테스트 1
    let loop_nums = [10, 20, 30, 40, 50];
    for element in loop_nums.iter() {
        println!("loop_nums : {}", element);
    }

    // 반복 테스트 2
    for rev_number in (1..4).rev(){
        println!("rev numbers : {}!", rev_number);
    }

}

fn jjw() -> i32 {
    1 + 2
}