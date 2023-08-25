use std::{  fs,error::Error};
use std :: env;

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config,&str>{
        if args.len() < 3 {
            return Err("---->Not enough arguments !!<----");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let fetch_env = env :: var("CASE_INSENSITIVE");
        let case_sensitive;
        match fetch_env {
            Ok(status) => {
                case_sensitive = if status == "true" {false} else {true};
            },
            Err(_) => {
                case_sensitive = true;
            }
        }
        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn logic(conf : &Config) -> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(conf.filename.as_str())?;
    let res;
    if conf.case_sensitive {
        res = search_case_sensitive(conf.query.as_str(),content.as_str());
    }else {
        res = search_case_insensitive(conf.query.as_str(),content.as_str());
    }
    for line in res {
        println!("{}",line);
    }
    Ok(())
}

pub fn search_case_sensitive<'a> (pattern : &str,contents : &'a str) -> Vec<&'a str> {
    let mut ans = Vec::new();
    for line in contents.lines() {
        if line.contains(pattern) {
            ans.push(line);
        }
    }
    ans
}

pub fn search_case_insensitive<'a> (pattern : &str,contents : &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut ans = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(pattern.as_str()) {
            ans.push(line);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let pattern = "abc";
        let text = "hello Abc !\nmiddle lines\nnot getting printed\nbye abc !";
        assert_eq!(vec!["bye abc !"],search_case_sensitive(pattern,text));
    }

    #[test]
    fn test_case_insensitive() {
        let pattern = "Abc";
        let text = "hello abc !\nmiddle lines\nnot getting printed\nbye abc !";
        assert_eq!(vec!["hello abc !","bye abc !"],search_case_insensitive(pattern,text));
    }
}