enum MultiType{
    IntType(i32),
    DoubleType(f64),
    TextType(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(12);
    v.push(34);
    v.push(56);

    println!("{}, {}, {}", v[0], v[1], v[2]);
    
    let v = vec![12,34,56];
    println!("{}, {}, {}", v[0], v[1], v[2]);

    // let is_exist:&i32 = &v[100]; // panic! 에러 발생.. 컴파일 시 잡기 어려움
    let is_exist:Option<&i32> = v.get(100);  // 있는 경우 Some, 없는 경우 None 반환

    match is_exist{
        Some(data) => println!("is data [{}]", data),
        _ => println!("is None data..."),
    }

    // mut 변수에 대해 immut 변수 참조 시 에러.
    // ===========================================
    // let mut v2:Vec<i32> = vec![1,2,3,4,5];
    // let refer_v = &v2[0];
    
    // v2.push(5);
    // print!("{}", refer_v);
    // ===========================================

    let for_vec:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    for each_vec in for_vec{
        print!("{}", each_vec);
    }

    let mut for_vec:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    for each_vec in &mut for_vec{
        *each_vec += 50;
    }

    println!("");

    let vector_multi_type:Vec<MultiType> = vec![
        MultiType::IntType(99),
        MultiType::DoubleType(3.1415),
        MultiType::TextType(String::from("This is text")),
    ];

    for each_item in vector_multi_type{
        match each_item{
            MultiType::IntType(i) => { println!("{}", i); },
            MultiType::DoubleType(i) => { println!("{}", i); },
            MultiType::TextType(i) => { println!("{}", i); },
        }
    }
}