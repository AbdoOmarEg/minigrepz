use std::error::Error;
use std::{env, fs};
// struct with & fileds, what does it mean, search for it
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //just takes input
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

//does the logic on the input receieved
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    // missing ; rust-analyzer error
    // let results = if config.ignore_case {
    //     search_case_insensitive(&config.query, &content); <-
    // } else {
    //     search(&config.query, &content);
    // };

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn _my_search<'a>(query: &str, conents: &'a str) -> Vec<&'a str> {
    let mut count = 0;
    for c in conents.lines() {
        for mut q in 0..query.len() {
            for ch in c.chars() {
                if query.chars().nth(q).unwrap() == ch {
                    count += 1;
                    q += 1;
                    if count == query.len() {
                        return vec![c.trim_start()];
                    }
                    if q == query.len() {
                        break;
                    }
                } else {
                    count = 0;
                }
            }
        }
    }

    vec![]
}

pub fn search<'a>(query: &str, conents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in conents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, conents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();

    for line in conents.lines() {
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

    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensative() {
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

// fn parse_confg(args: &[String]) -> (&str, &str) {
//     // let query: &String = &args[1];
//     // let file_path: &String = &args[2];
//     let query = &args[1]; //&str
//     let file_path = &args[2]; //&str
//
//     // what's the diff between &String and &str in this context
//     (query, file_path)
// }
