fn main() {
    // segmatics 의미론 예제 ===========================
    // 의미론이란 : 값을 변수에 대입하는 것과 유사함.
    // 즉, 함수에 값을 전달하는 것은 변수값을 넘기는 방식과 유사함 (copy 하거나 move 하거나. call by value, call by reference 같은 의미?)

    let x = 5;
    makes_copy(x);
    // 해당 함수는 문제가 없다. i32 등은 ownership 에서 말했듯이 Copy 트레잇을 가지고 있다.

    let s = String::from("segmatics string");
    takes_ownership(s);
    // 해당 함수는 문제가 생길 수 있다.
    // s 값은 drop 트레잇을 가지고 있으므로 함수 밖으로 벗어나는 시점에서 스코프 밖으로 벗어나면서 해제된다.
    // println!("{}", s);  // 에러!!


    // ==============================================
    let _get_owner_string = gives_ownership();   // 함수 내에서 만든 some_string 소유권을 _get_owner_string으로 이동
    println!("{}", _get_owner_string);  // 문제없음!
    
    let _give_string = String::from("give");
    println!("{}", _give_string);  // 문제없음!
    
    let _take_string = take_and_gives_back(_give_string);
    // println!("{}", _give_string);  // 소유권 이동됨! 에러!
    println!("{}", _take_string);  // 소유권 이동됨! 정상!
    
    // ==============================================
    // 매 번 소유권을 신경쓰기 너무 귀찮은데요...   >> 추후 참조자를 배웁니다 (reference)
    // 대신에, 여러 값을 반환받을 수도 있습니다.
    let _input_string = String::from("input?");
    let (_output_string, length) = calc_length(_input_string);
    println!("{} length is {}", _output_string, length);
}

// ==============================================
fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}
// ==============================================
fn gives_ownership() -> String{
    let some_string = String::from("give owner");
    some_string
}

fn take_and_gives_back(tagb_string:String) -> String{
    tagb_string
}
// ==============================================
fn calc_length(_input_string:String) -> (String, usize){
    let length = _input_string.len();
    (_input_string, length)
}