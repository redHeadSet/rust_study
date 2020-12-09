use std::collections::HashMap;

fn main() {
    let mut valuelist = vec![1, 2, 3, 3, 5];

    let mut mean = 0;
    let length = &valuelist.len();

    // 평균값
    for total in &valuelist {
        mean += total;
    }
    mean = mean / &valuelist.len();
    println!("평균값 : {}", mean);

    // 중간값
    &valuelist.sort();
    let midindex = (0 + length-1) / 2;
    let median = &valuelist[midindex];
    println!("중간값 : {}", median);

    // 최빈값
    // map에 count 체크해서 넣어놓고
    let mut map = HashMap::new();
    for val in &valuelist {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }

    // 가장 높은 count의 value 값을 출력함
    let mut modevalue = 0;
    let chkcount = 1;
    for (key, value) in &map {
        if chkcount < *value {
            modevalue = **key;
        }
    }
    println!("최빈값 : {}", modevalue);
}
