use std::collections::HashMap;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut text= String::from("");
    io::stdin().read_line(&mut text).expect("ERRROR");
    // let  text = String::from("first");
    
    //자음 = 단어의 끝
    // 모음 = 끝에 hay
    let c = &text[..1];
    let   text :String = text.trim().parse().expect("ERRORRR");
    let  result;
    match c {
        "a" | "i"| "u"| "e" | "o" => {
            result = AIUEO(text);
        },
        _ => {
            result = O(text);
        },
    }

    println!("{}",result);

    //벡터를 이용하여 
    let list = vec![1, 2, 3, 4, 1, 2 , 3, 1]; 
    let mut sum = 0;
    //리스트의 평균값
    for i in &list {
        sum += i;
    }
    let a :f32 =sum as f32/ list.len()as f32 ;
    println!("평균값 {}",a);

    
    // 중간값
    let mut list2 = list.clone();
    list2.sort();
    println!("{:?}",list2);
    let mid = list2[list.len() /2];
    println!("중간값{}", mid);
    //match mid {
    //    Some(x) => {
    //        println!("중간값{}", x);
    //    },
    //    _ => {},
    //}


    // 최빈값 반환
    let mut hash = HashMap::new();
    for i in &list {
        let count = hash.entry(i).or_insert(0); // 가변참조자
        *count += 1;
    }

    println!("{:?}", hash);

    let mut check_value = 0;
    let mut num = 0;
    for (key, value) in hash.iter(){
        println!("{}", value);
        match value.cmp(&num) {
            Ordering::Greater    => {
                num = *value;
                check_value = **key;
            }
            _ => {}
        }
    }
    println!("제일 많이 나온 수 {}", check_value);

    //======================
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue", 10));
    // scores.insert(String::from("Yellow", 50));
     let teams = vec![String::from("Blue"),String::from("Yellow")];
     let initial_scores = vec![10, 50];

     let mut scores : HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
     let red_team = String::from("Red");
    let mut team_name = String::from("Blue");
    //let score = scores.get(&team_name);
        
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    scores.entry(&red_team).or_insert(&100);
    scores.entry(&team_name).or_insert(&70);

    println!("{:?}",scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 가변참조자
        *count += 1;
    }
    println!("{:?}",map);
    

    ///////////////3번
    let mut p_map = HashMap::new();
    let mut v1 = vec![];
    p_map.clear();
    v1.clear();
    loop{
        let mut text3= String::new();
        io::stdin().read_line(&mut text3).expect("ERRROR : get");

        text3= text3.trim().parse().expect("ERRORRR : trim"); // Text로 변경
        text3= text3.replace("Add", ""); // Add는 안쓰니까
        let name:Vec<String> = text3.split("to").map(String::from).collect();
        let num = name.len();
        
        match num {
            1 => {
              let first = String::from(&name[0]);
              for elem in v1.clone() {
                  if first == elem {
                      println!("부서 - {}",first);
                      let mut v2 = vec![];
                      for (key,value) in p_map.clone() {
                        if value == elem{
                              v2.push(key);
                        }
                      }
                      v2.sort();
                      println!("부서내 명단 : {:?}",v2);
                  }
              }
              //모든 사람 내뱉기 
              let mut v2 = vec![];
              for i in p_map.keys(){
                  v2.push(i);
              }
              v2.sort();
              println!("사람 : {:?}", v2);
            }
            2 => {
                println!("{}",name[0]);
                println!("{}",name[1]);
                p_map.insert(String::from(name[0].trim()),String::from(name[1].trim()));
                println!("지금까지 저장된 리스트 : {:?}",p_map);
        
                for a in p_map.values(){
                    v1.push(a.clone());
                }
                v1.sort(); //줄세우기
                v1.dedup(); // dup 제거
            }
            _ => {
                println!("Input 값을 확인해주세요.");
            }
        }
    }
}

fn AIUEO(s:String) -> String{
    let s1 = s + "-hay";
    //let s1 = s.push_str("-hay");
    s1
}

fn O(s:String) -> String{
    let s1 = format!("{}-{}ay", s, &s[0..1]);
    let s2 = String::from(&s1[1..]);
    s2
}
