use std::env;
use std::fs;
use std::process;

mod util;

use util::Config;
use util::search;   

fn parse_config(all_args: &Vec<String>) -> Result<Config, &'static str> {
    if all_args.len() < 3 {
        return Err("not enough arguments");
    }
    let query = all_args[1].clone();
    let filename = all_args[2].clone();
    Ok(Config {
        query,
        filename
    })
}

fn read_file(filename: &String) -> Result<String, &'static str> {
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(s) => Ok(s),
        Err(_) => Err("no such file or directory"),
    }
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let config = match parse_config(&all_args) {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };
    let content = match read_file(&config.filename) {
        Ok(content) => content,
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };
    let result = search::search_str(&config.query, &content);
    println!("{:?}", result);
}
