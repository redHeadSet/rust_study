fn main() {
    println!("Hello, world!");
    //========================
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    //  print!("{}, {}", s1,s2);
   
    //========================
//     let mut s = String::from("hello");
//    //let r1 = &mut s;
//    //let r2 = &mut s;
//    //println!("{}",r2);
//    //println!("{}",r1);
//    let r1 = &s;
//    let r2 = &s;
//    println!("{}",r2);
//    //println!("{}",r1);
   
   //========================
//    let s1 = String::from("hello");
//    let len = calculate_length(&s1);
//    println!("{}",len);
//    println!("{}",s1);

   //========================
//    let a = no_dangling();
//    println!("a = {}",a);
  //=========================
  //스트링 리터럴 하드코딩 문자열값은 불변
  let mut s = String::from("WOOOOOOOW YEAH");
  let word = first_word(&s);
  //s.clear();
  println!("size {}",word.len());

  let slice = &s[0 .. 3];
  println!("1. {}",slice);
  
//   let slice = s[0 .. 3];  // str the size for values of type `str` cannot be known at compilation time
//   println!("1. {}",String::from(&slice));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn no_dangling() -> String {
    let s = String::from("helllo");
    s
}
// fn first_word(s: &String) -> usize {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i; //return 사용 가능.
            return &s[0..i];
        }
    }
    // s.len()
    &s[..]
}
