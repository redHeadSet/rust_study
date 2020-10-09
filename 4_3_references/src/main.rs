fn main() {
    let str_len_test = String::from("length test");
    let _local_length = str_len_test.len();

    let _function_length = calculate_length(&str_len_test);

    println!("str_len_test : {}", str_len_test);    // & 사용하여 전달하면 소유권이 아닌, 레퍼런스가 전달되어 사용 가능.
                                                    // 레퍼런스를 사용해 전달하면 소유권은 전달되지 않으므로 해당 원본 값은 변경 불가.
    println!("local length : {}", _local_length);
    println!("function length : {}", _function_length);

    // =====================================================

    let mut modify_str_test = String::from("modify string test");
    println!("{}", modify_str_test);
    modify_str(&mut modify_str_test);   // 함수 내에서 String 값을 바꾸고 싶다면 '당연히' mutable 선언을 해주어야 한다.
    println!("{}", modify_str_test);

    // =====================================================
    
    let mut mutable_string_test = String::from("mutable string test");
    modify_str(&mut mutable_string_test);
    println!("mutable_string_test : {}", mutable_string_test);
    let ref_1 = &mut mutable_string_test;
    modify_str(ref_1);
    println!("ref_1 : {}", ref_1);
    let ref_2 = &mut mutable_string_test;
    modify_str(ref_2);
    println!("ref_2 : {}", ref_2);
    // println!("ref_1 : {}", ref_1);   // 에러!

    // =====================================================
    
    let mut ref_error_test = String::from("ref_error_test");
    let ref_1 = &ref_error_test;
    let ref_2 = &ref_error_test;        // 여러 개의 불변 참조자는 가능.
    // let ref_3 = &mut ref_error_test; // 불변 참조자가 있을 때 
    println!("ref_1 : {}", ref_1);
    println!("ref_2 : {}", ref_2);
    // println!("ref_3 : {}", ref_3);

    // =====================================================

    let ref_dangle_test = dangle(); // 아래 dangle 함수 참고
}

fn calculate_length(input_string: &String) -> usize{
    input_string.len()
}

fn modify_str(input_string: &mut String) {  // 함수 내에서 String 값을 바꾸고 싶다면 '당연히' mutable 선언을 해주어야 한다.
    input_string.push_str(" modify!!");
}

// fn dangle() -> &String{
//     let s = String::from("dangle string");
//     &s   // &s로 값 전달(레퍼런스 전달)한 후, s 값은 삭제됨(drop 트레잇) 즉, &s 값은 알 수 없는 값을 참조하게 되므로 에러.
// }

fn dangle() -> String{
    let s = String::from("dangle string");
    s   // 직접 값을 전달하게 되면서 소유권 전달. (정상적)
}