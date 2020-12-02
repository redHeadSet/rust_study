// 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 평균값(mean, average), 
// 중간값(median, 정렬했을 때 가장 가운데 위치한 값), 
// 그리고 최빈값(mode, 가장 많이 발생한 값; 해쉬맵이 여기서 도움이 될 것입니다)를 반환해보세요.

use std::io;
use std::collections::HashMap;

struct ListManager{
    vec_inputs:Vec<i32>,
    inputs_count:HashMap<i32,i32>,
}

impl ListManager{
    fn insert(&mut self, num:i32){
        self.vec_inputs.push(num);
        let count = self.inputs_count.entry(num).or_insert(0);
        *count += 1;
    }

    fn printing(&self){
        println!("Vecotr : {:?}", self.vec_inputs);
        println!("Hash : {:?}", self.inputs_count);
    }

    fn average(&mut self) -> f64 {
        let mut sum:f64 = 0.0;
        let vec_size:f64 = self.vec_inputs.len() as f64;

        for each_num in &self.vec_inputs{
            sum += *each_num as f64;
        }

        sum / vec_size
    }

    // fn sort(&mut self) {
    //     let mut new_vec_inputs:Vec<i32> = Vec::new();

    //     for orig_each in &self.vec_inputs{
    //         // i 는 0부터 self.vec_inputs.len()-1 까지
    //         let mut inputs_index = 0;
    //         for (j, new_each) in new_vec_inputs.iter().enumerate(){
    //             inputs_index = j;
    //             if *new_each > *orig_each { break; }
    //             else if j == new_vec_inputs.len()-1 { inputs_index += 1; break; }
    //         }
    //         new_vec_inputs.insert(inputs_index, *orig_each);
    //     }

    //     println!("new vec : {:?}", new_vec_inputs);
    //     self.vec_inputs = new_vec_inputs;
    // }

    fn middle_value(&self) -> i32{
        let mut new_vec_inputs:Vec<i32> = Vec::new();

        for orig_each in &self.vec_inputs{
            // i 는 0부터 self.vec_inputs.len()-1 까지
            let mut inputs_index = 0;
            for (j, new_each) in new_vec_inputs.iter().enumerate(){
                inputs_index = j;
                if *new_each > *orig_each { break; }
                else if j == new_vec_inputs.len()-1 { inputs_index += 1; break; }
            }
            new_vec_inputs.insert(inputs_index, *orig_each);
        }

        println!("sorting vec : {:?}", new_vec_inputs);

        match new_vec_inputs.get(self.vec_inputs.len() / 2){
            Some(i) => *i,
            _ => 0,
        }
    }

    fn most_count_num(&self) -> i32{
        let mut most_key = 0;
        let mut most_num = 0;
        for (each_key, each_num) in &self.inputs_count{
            if most_num < *each_num {
                most_key = *each_key;
                most_num = *each_num;
            }
        }
        most_key
    }
}

fn main() {
    println!("go!");
    let mut input_list = ListManager{
        vec_inputs:Vec::new(),
        inputs_count:HashMap::new(),
    };

    loop {
        let mut inputs = String::new();

        io::stdin().read_line(&mut inputs)
        .expect("Failed to read line");
    
        let input_data:Option<i32> = match inputs.trim().parse() {
            Ok(num) => Some(num),
            _ => None,
        };
    
        match input_data{
            Some(num) => {
                input_list.insert(num);
            },
            _ => {
                println!("is not data...");
                break;
            },
        }
    }

    input_list.printing();
    println!("average is [{}]", input_list.average());
    println!("middle value is [{}]", input_list.middle_value());
    println!("most key is [{}]", input_list.most_count_num());
}