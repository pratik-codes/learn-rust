use std::env;
use std::fs;
use std::io::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// parse the args to get config
// using clone we loose a little on speed and performance but we gain on code redablitiy by not
// introducing lifetimes
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn get_command_line_args() -> Vec<String> {
    // Collects the command-line arguments into a vector
    env::args().collect()
}

pub fn read_file_content(file_path: &String) -> Result<String, Error> {
    let content = fs::read_to_string(file_path);
    match content {
        Ok(data) => Ok(data),
        Err(err) => Err(err), // Propagate the error
    }
}
