use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn get_command_line_args() -> Vec<String> {
    // Collects the command-line arguments into a vector
    env::args().collect()
}

// parse the args to get config
pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

pub fn read_file_content(file_path: &String) -> String {
    let content = fs::read_to_string(file_path).expect("should have been able to read the file");
    content
}
