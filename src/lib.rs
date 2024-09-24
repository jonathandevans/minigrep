use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath())?;

    let res = if config.ignore_case() {
      search_case_insensitive(config.query(), &contents)
    } else {
      search(config.query(), &contents)
    };

    for line in res {
      println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut res: Vec<&str> = Vec::new();

  for line in contents.lines() {
      if line.to_lowercase().contains(&(query.to_lowercase())) {
          res.push(line);
      }
  }

  res
}

pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let filepath: String = args[2].clone();

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, filepath, ignore_case })
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}