mod args;
use args::cli_options;

use std::fs;
use std::io::{self, Read};
use std::ops::Add;

use regex::Regex;

use clap::ArgMatches;

use colored::{Color, Colorize};

fn parse_args_regex(args: &ArgMatches) -> Vec<(Regex, Color)> {
    let mut custom_regex = Vec::new();
    let default_regex = [
        (r"(?P<r>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})", Color::Red), // IP
        (r"(?P<r>(?i)password)", Color::Blue),                      // Password
        (r"(?P<r>(?i)login)", Color::Blue),                         // Login
        (r"(?P<r>(?i)credentials?)", Color::Blue),                  // Credentials
        // (r"(?P<r>(?:[a-zA-Z0-9+/]{4})*(?:|(?:[a-zA-Z0-9+/]{3}=)|(?:[a-zA-Z0-9+/]{2}==)|(?:[a-zA-Z0-9+/]{1}===)))", Color::Cyan), // a base64 encoded string TO MANY FALSE POSITIVE
        (r"(?P<r>(?i)[a-f0-9]{32})", Color::Red), // a hash
    ];
    let pattern_begin = String::from("(?P<r>");

    if !args.is_present("no-default") {
        for (r, c) in default_regex {
            custom_regex.push((Regex::new(r).unwrap(), c))
        }
    }

    if let Some(r) = args.value_of("color-red") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Red))
        }
    }
    if let Some(r) = args.value_of("color-blue") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Blue))
        }
    }
    if let Some(r) = args.value_of("color-green") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Green))
        }
    }
    if let Some(r) = args.value_of("color-yellow") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Yellow))
        }
    }
    if let Some(r) = args.value_of("color-magenta") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Magenta))
        }
    }
    if let Some(r) = args.value_of("color-cyan") {
        let split: Vec<&str> = r.split(',').collect();
        for r in split {
            let mut t = r.to_string();
            t.insert_str(0, &pattern_begin);
            t = t.add(")");
            custom_regex.push((Regex::new(t.as_str()).unwrap(), Color::Cyan))
        }
    }
    custom_regex
}

fn process_regex(all_regexes: Vec<(Regex, Color)>, buffer: &mut String) {
    for (r, c) in all_regexes {
        match c {
            Color::Red => *buffer = r.replace_all(buffer, "$r".red().to_string()).to_string(),
            Color::Blue => *buffer = r.replace_all(buffer, "$r".blue().to_string()).to_string(),
            Color::Green => *buffer = r.replace_all(buffer, "$r".green().to_string()).to_string(),
            Color::Yellow => *buffer = r.replace_all(buffer, "$r".yellow().to_string()).to_string(),
            Color::Magenta => {
                *buffer = r
                    .replace_all(buffer, "$r".magenta().to_string())
                    .to_string()
            }
            Color::Cyan => *buffer = r.replace_all(buffer, "$r".cyan().to_string()).to_string(),
            _ => {}
        }
    }
}

fn main() {
    let cli = cli_options();

    let all_regex = parse_args_regex(&cli);

    let mut buffer = String::new();
    let mut stdin = io::stdin();

    if let Some(file) = cli.value_of("file") {
        buffer = fs::read_to_string(file).expect("Error while reading the file");
    } else {
        stdin
            .read_to_string(&mut buffer)
            .expect("Error while reading stdin");
    }

    process_regex(all_regex, &mut buffer);

    print!("{}", buffer);
}
