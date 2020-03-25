use std::env;
use std::fs;
use std::process;

mod util;

use util::Config;
use util::search;   

fn parse_config(all_args: &Vec<String>) -> Config {
    let config = Config::new(all_args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    config
}

fn read_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    contents
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let config = parse_config(&all_args);
    let content = read_file(&config.filename);
    let result = search::search_str(&config.query, &content);
    println!("{:?}", result);
}
