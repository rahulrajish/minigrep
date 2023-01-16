use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case_env = env::var("IGNORE_CASE").or_else(|_| 
            Ok(String::from("Ok"))
        )?;

        let ignore_case = get_ignore_case(&args, &ignore_case_env);

        Ok(Config { query, file_path, ignore_case })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };


    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a> (query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

pub fn get_ignore_case(args: &[String], env_args: &str) -> bool {
    let mut ignore_case = false;
    if args.len() > 3 {
        let ignore_case_args = args[3].clone();
        if ignore_case_args == "Ignore_Case" {
            ignore_case = true;
        } 
    }

    if env_args == "1" {
        ignore_case = true;
    } else if env_args == "0" {
        ignore_case = false;
    }
    ignore_case
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn ignore_case_env_is_true() {
        let ignore_case_cmd:Vec<String> = vec!["arg1".to_string(),"arg2".to_string(),"arg3".to_string(),"Ignore_Case".to_string()];
        let ignore_case_env = "1";
        assert_eq!(true, get_ignore_case(&ignore_case_cmd, ignore_case_env));
    }

    #[test]
    fn ignore_case_env_is_false() {
        let ignore_case_cmd:Vec<String> = vec!["arg1".to_string(),"arg2".to_string(),"arg3".to_string(),"Ignore_Case".to_string()];
        let ignore_case_env = "0";
        assert_eq!(false, get_ignore_case(&ignore_case_cmd, ignore_case_env));
    }
}