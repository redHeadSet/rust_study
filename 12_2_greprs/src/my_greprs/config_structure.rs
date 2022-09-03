pub struct GrepConfigs{
    query: String,
    filename: String,
}

impl GrepConfigs {
    pub fn parse_args(args: &[String]) -> Result<GrepConfigs, &'static str> {
        if args.len() < 3 {
            return Err("more arguments");
        }
        Ok(GrepConfigs {
            // clone 을 사용하는 것에 대한 기회비용을 생각
            // 추후 &str 값으로 바꾸면 어떻게 해야 할 지 고민
            query : args[1].clone(),
            filename : args[2].clone()
        })
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}