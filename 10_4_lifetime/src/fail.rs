// 댕글링 참조자 예시
fn dangling_ref(){
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
// ================================================================

fn longest<'l>(x: &'l str, y: &'l str) -> &'l str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 이 함수를 컴파일하면, result 의 lifetime이 적절치 않아 에러가 발생한다
fn lifetime_err() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// ================================================================
// 근데 이건 왜 되는건지 모르겠다...?
fn lifetime() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}