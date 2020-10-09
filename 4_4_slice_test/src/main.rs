fn main() {
    let mut calc_string = String::from("make calculating string!!");

    println!("String : {}", calc_string);
    println!("first space index : {}", first_space_index(&calc_string));

    let calced_string = first_word(&calc_string);
    println!("first word : {}", calced_string);

    calc_string.clear();
    // println!("first word : {}", calced_string);  // 당연히, slice도 원본의 '일부 참조'이므로 원본을 삭제하는 경우 문제가 생김.

    let i32_origin = [1, 2, 3, 4, 5];
    let i32_slice = &i32_origin[1..3];
    for (i, item) in i32_slice.iter().enumerate(){
        println!("{} : {}", i, item);
    }
}

fn first_space_index(input_string: &String) -> usize{
    let bytes = input_string.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        // enumerate : iter의 각 결과값을 item 내에 매핑함. 즉, i는 횟수, item은 해당 값
        if item == b' ' {
            return i;
        }
    }

    input_string.len()
}

fn first_word(input_string: &String) -> &str{   // &str 타입 = 스트링 리터럴 타입. 또한 string slice 는 &str 타입이다.
                                                // &str 타입이 c++의 char* 형식과 비슷해보임...
                                                // ex) let s:&str = "abc"; << 스트링 슬라이스 (즉, 해당 값은 불변일 수 밖에 없음...)
    let bytes = input_string.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_string[0..i];
        }
    }

    &input_string[..]   // input_string의 처음부터 끝까지의 참조를 전달... 아래와 사실상 비슷할 거 같은데 잘 모르겠음
    // &input_string    // input_string의 참조를 전달...?
}