use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct GrepConfigs{
    query: String,
    filename: String,
    contents: String,
}

impl GrepConfigs {
    pub fn get_query(&self) -> &str { &self.query }
    pub fn get_filename(&self) -> &str { &self.filename }

    pub fn new_config(query:&str, filename:&str)
        -> Result<GrepConfigs, &'static str> {
        if query.len() <= 0 || filename.len() <= 0 {
            return Err("must length over 0");
        }
        Ok( GrepConfigs{
            query: query.to_string(),
            filename: filename.to_string(),
            contents: String::new(),
        })
    }

    pub fn parse_args(args: &[String]) -> Result<GrepConfigs, &'static str> {
        if args.len() < 3 {
            return Err("more arguments");
        }
        Ok(GrepConfigs {
            query: args[1].clone(),
            filename: args[2].clone(),
            contents: String::new(),
        })
    }

    pub fn read_file(&self) -> Result<String, Box<dyn Error>> {
        let filename = &self.filename;

        let mut f = match File::open(filename){
            Ok(file) => file,
            Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
                panic!("file not found")
            },
            Err(error) => {
                panic!("some error {:?}", error)
            },
        };

        let mut contents = String::from("");

        f.read_to_string(&mut contents)?;

        // println 이 아닌 eprintln 인 경우, 표준 에러로 출력된다
        eprintln!("contents : {}", contents);

        Ok(contents)
    }

    pub fn search_on_contents<'a>(&self, contents:&'a str) -> Vec<&'a str>{
        let mut rtn_vec = Vec::new();

        for line in contents.lines() {
            if line.contains(&self.query){
                rtn_vec.push(line.clone());
            }
        }
        rtn_vec
    }

    pub fn load_contents(&mut self) {
        if self.contents.len() <= 0 {
            self.contents = self.read_file().unwrap_or_else(|err| {
                panic!("load contest error {}", err);
            })
        }
    }

    pub fn search(&mut self) -> Vec<&str> {
        self.load_contents();

        // let mut rtn_vec : Vec<&str> = vec![];
        let mut rtn_vec = Vec::new();
        for line in self.contents.lines() {
            if line.contains(&self.query){
                rtn_vec.push(line.clone());
            }
        }
        rtn_vec
    }

    pub fn search_nocase(&mut self) -> Vec<String> {
        self.load_contents();
        let nocase_query = self.query.to_ascii_lowercase();
        let nocase_contents = self.contents.to_ascii_lowercase();

        let mut rtn_vec = Vec::new();
        for line in nocase_contents.lines() {
            if line.contains(&nocase_query){
                rtn_vec.push(String::from(line));
            }
        }
        rtn_vec
    }
}