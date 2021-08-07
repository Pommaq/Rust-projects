use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err(String::from("Usage: query filename"));
        }

        return Ok(Config {
            query: String::from(&args[1]),
            filename: String::from(&args[2]),
            case_sensitive: std::env::var("CASE_INSENSITIVE").is_err(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results;
    if config.case_sensitive {
        results = search(&config.query, &contents);
    } else {
        results = search_case_insensitive(&config.query, &contents);
    }

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
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
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
        let query = "ruST";
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
