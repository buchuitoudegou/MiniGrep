pub mod search;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(all_args: &Vec<String>) -> Result<Config, &'static str> {
        if all_args.len() < 3 {
            Err("no enough arguments")
        } else {
            Ok(Config {
                query: all_args[1].clone(),
                filename: all_args[2].clone(),
            })
        }
    }
}
