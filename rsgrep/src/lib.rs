use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        args.next();

        let query = args.next().expect("Didn't get a query string");
        let filename = args.next().expect("Didn't get a file name");

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(cfg.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = match cfg.case_sensitive {
        true => search(&cfg.query, &contents),
        false => search_case_insensitive(&cfg.query, &contents),
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // let mut matched_strings = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         matched_strings.push(line);
    //     }
    // }
    //
    // matched_strings
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
