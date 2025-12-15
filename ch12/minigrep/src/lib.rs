use std::fs;
use std::env;
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Config{
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path, ignore_case: false } // query and file_path have same names as struct fields
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case})
    }
    
    pub fn query(&self) -> &str {
        &self.query
    }
    
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path())?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query(), &contents)
    } else {
        search(&config.query(), &contents)
    };
    // println!("With text:\n{contents}");
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, proDuctive.
Pick three.";
        assert_eq!(vec!["safe, fast, proDuctive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}