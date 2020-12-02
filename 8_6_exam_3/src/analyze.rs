
#[derive(Debug)]
pub struct AnalyzeData{
    pub validity: bool,
    pub command:String,
    pub name: String,
    pub department: String,
}

pub fn analyze_input_string(input_string:&String) -> AnalyzeData {
    let mut out_data = AnalyzeData{
        validity:false,
        command:String::new(),
        name:String::new(),
        department:String::new(),
    };

    for (word_count, each_word) in input_string.trim().split_whitespace().enumerate(){
        match word_count{
            0 => {
                if let false = match each_word.to_ascii_uppercase().as_ref() {
                    "ADD" | "REMOVE" => {
                        out_data.command = format!("{}", each_word.to_ascii_uppercase());
                        true
                    },
                    "QUIT" => {
                        out_data.command = format!("{}", each_word.to_ascii_uppercase());
                        false
                    },
                    "LIST" => {
                        out_data.command = format!("{}", each_word.to_ascii_uppercase());
                        false
                    },
                    _ => false,
                }{
                    println!("first data : {}", out_data.command);
                    if let "QUIT" = out_data.command.as_ref(){
                        out_data.validity = true;
                    }
                    else if let "LIST" = out_data.command.as_ref(){
                        out_data.validity = true;
                    }
                    else{
                        println!("명령 확인 필요 : add | remove | quit | list");
                    }
                    break;
                }
            },
            1 => {
                out_data.name = format!("{}", each_word);
            },
            2 => {
                if let false = match each_word.to_ascii_uppercase().as_ref(){
                    "TO" => true,
                    _ => false,
                }{
                    println!("확인 필요 : to");
                    break;
                }
            },
            3 => {
                out_data.department = format!("{}", each_word);
                out_data.validity = true
            },
            _ => {
                out_data.validity = false;
                break;
            },
        }
    }

    out_data
}
