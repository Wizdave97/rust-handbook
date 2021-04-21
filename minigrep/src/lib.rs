pub mod helpers {
    use std::{env, fs};
    use std::error::Error;
    #[derive(Debug)]
    pub struct Config<'a> {
        query: &'a str,
        filename: &'a str,
        case_insensitive: bool
    }
    
    impl<'a> Config<'a> {
        pub fn new<'b>(args: &'b [String]) -> Result<Config<'b>, String> {
            let mut args = args.iter();
            let query = args.next();
            let filename = args.next();
            if query.is_none() {
                return Err("Search query was not supplied".to_string());
            }
            if filename.is_none() {
                return Err("File name was not supplied".to_string());
            }
            let case_insensitive = env::var("CASE_INSENSITIVE").unwrap_or_else(|_err | "false".to_string());
            Ok(Config { query: query.unwrap(), filename: filename.unwrap(), case_insensitive: case_insensitive == "true" })
        }

        pub fn query(&self) -> &str {
            &self.query
        }

        pub fn filename(&self) -> &str {
            &self.filename
        }

        pub fn case_insensitive(&self) -> bool {
            *&self.case_insensitive
        }
    }
    
    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(config.filename())?;
        if config.case_insensitive() {
            for line in search_case_insenitive(config.query(), &content) {
                println!("{}", line);
            }
        }
        else {
            for line in search(config.query(), &content) {
                println!("{}", line);
            }
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let result_vec: Vec<&str> = content.lines().filter(| line | line.contains(query)).collect();
        result_vec
    }

    pub fn search_case_insenitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let result_vec: Vec<&str> = content.lines().filter(| line | line.to_lowercase().contains(&query.to_lowercase())).collect();
        result_vec
    }
}