// 해쉬맵과 벡터를 이용하여,
// 사용자가 회사 내의 부서에 대한 피고용인 이름을 추가할 수 있도록 하는 텍스트 인터페이스를 만들어보세요.
// 예를 들어 “Add Sally to Engineering”이나 “Add Amir to Sales” 같은 식으로요.
// 그후 사용자 각 부터의 모든 사람들에 대한 리스트나 알파벳 순으로 정렬된 부서별 모든 사람에 대한 리스트를 조회할 수 있도록 해보세요.

mod analyze;

use std::io;
use analyze::*;
use std::collections::HashMap;

fn main() {
    let mut company:HashMap<String, Vec<String>> = HashMap::new();

    loop{
        let mut input_command = String::new();
        println!();
        println!("input command : [quit] 시 종료");
        println!("input command : [list] 시 리스트업");

        io::stdin().read_line(&mut input_command)
        .expect("input command error");
        
        let analyze_data = analyze_input_string(&input_command);
        println!("return : {:?}", analyze_data);    // 데이터 확인용
        
        if let "QUIT" = analyze_data.command.as_ref(){
            println!("종료합니다.");
            break;
        }

        if let false = analyze_data.validity{
            continue;
        }

        match analyze_data.command.as_ref(){
            "ADD"    => add_employee(&mut company, analyze_data),
            "REMOVE" => remove_employee(&mut company, analyze_data),
            "LIST"   => view_company(&company),
            _ => {},
        };

        
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, analyze_data: AnalyzeData)
{
    match company.get_mut(&analyze_data.department){
        Some(employees) => {
            employees.push(analyze_data.name);
        },
        None => {
            let mut v_emplyees:Vec<String> = Vec::new();
            v_emplyees.push(analyze_data.name);
            company.insert(analyze_data.department, v_emplyees);
        },
    }
}

fn remove_employee(company: &mut HashMap<String, Vec<String>>, analyze_data: AnalyzeData)
{
    match company.get_mut(&analyze_data.department){
        Some(employees) => {
            employees.remove( find_index(analyze_data.name.as_ref(), employees) );

            if let 0 = employees.len() {
                company.remove(&analyze_data.department);
            }
        },
        None => {
            println!("삭제할 애가 없어여");
        },
    }
}

fn find_index(find_name: &str, source : &Vec<String>) -> usize{
    let mut index = 0;
    for each_name in source{
        if each_name == find_name{
            break;
        }
        index = index+1;
    }
    index
}

fn view_company(company: &HashMap<String, Vec<String>>) {
    for (each_department_name, department_employee) in company {
        println!();
        println!("[{}] 부서", each_department_name);
        for each_employee in department_employee{
            print!("[{}]", each_employee);
        }
    }
    println!();
}