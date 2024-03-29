use std::error::Error;
use std::fs;
use std::env;

pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
    pub case_insensitive: bool,
}
impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> { // 'static can be ignore.
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config { query: &args[1], filename: &args[2], case_insensitive: env::var("CASE_INSENSITIVE").is_err() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if results.len() == 0 { println!("No result found"); }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase(); // turn to lower case,have ownership.

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
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
Duck tape.";

    assert_eq!(
        vec!["safe, fast, productive."], search(query, contents));
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