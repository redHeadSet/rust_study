use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct SplitStr{
    command : String,    
    department : String,
    name : String,
}

impl SplitStr{
    fn new() -> SplitStr {
        SplitStr {
            command : String::from(""),
            department : String::from(""),
            name : String::from(""),
        }
    }
    fn check(&self) -> bool {
        //match &self.command[..] {
        match self.command.as_str() {
            "Search" => true,
            "Add"=>{
                match self.department.as_str() {
                    "" => false,
                    _ => {
                        match self.name.as_str() {
                            "" => false,
                             _ => true,
                        }
                    },
                }
            }
            _ => false,
        }
    }
    fn split_str(mut s:String) -> SplitStr {
        let mut  new_split_str : SplitStr = SplitStr::new();
        let mut iter = s.split_whitespace();
        match iter.next(){
            Some(x) => {
                new_split_str.command =String::from(x);
                s = s.replace(x, "");
            }
            _ => println!("Command 값을 입력하세요")
        }
        
        let mut iter :Vec<String> = s.split("to").map(String::from).collect();
        match iter.len() {
            1 => {
                match iter.get_mut(0){
                    Some(x) => new_split_str.department = x.trim().to_string(),
                    _ => println!("department 입력 오류"),
                }
            }
            2 => {
                match iter.get_mut(0) {
                    Some(x) => new_split_str.name = x.trim().to_string(),
                    _ => println!("name 입력 오류"),
                }
                
                match iter.get_mut(1) {
                    Some(x) => new_split_str.department = x.trim().to_string(),
                    _ => println!("department  입력 오류"),
                }
            }
            _ => println!("Please write department and name"),
        }
        new_split_str
    }
}

fn main() {
    ///////////////3번
    let mut p_map = HashMap::new();
    //let mut vector_name = vec![];
    p_map.clear();
    //vector_name.clear();
    loop{
        println!("Command :
        1) Add [Name] to [Department]
        2) Search
        3) Search [Department]");
        let mut text= String::new();
        io::stdin().read_line(&mut text).expect("ERRROR : get");

        text= text.trim().parse().expect("ERRORRR : trim"); // Text로 변경
        let split_text : SplitStr = SplitStr::split_str(text);
//        println!("{:?}",split_text);
        if let false =  split_text.check() {
            println!("값을 제대로 써주세용");
        } else {
            match split_text.command.as_str(){
                "Search" => {
                    if let "" = split_text.department.as_str() {
                        println!("{:?}",p_map);
                        //let a :Vec<String> = p_map.clone().keys().map(String::from).collect();
                    } else{
                        let mut a = vec![];
                        for (key, value) in p_map.clone() {
                            match value == split_text.department  {
                                true => {
                                    a.push(key);
                                },
                                _ => {}
                            }
                        }
                        a.sort();
                        a.dedup();
                        for elem in a {
                            println!("{}",elem);
                        }
                    }  
                }                    
                "Add" => {
                    p_map.insert(split_text.name, split_text.department);
                    println!("{:?}",p_map);
                }
                _ =>{println!("command ERROR");}
            }
        }
    }
}
