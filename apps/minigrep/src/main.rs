use std::process;

mod utils;

fn main() {
    println!("Mini grep by Pratik...");

    let args: Vec<String> = utils::get_command_line_args();
    let config = utils::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let file_content = match utils::read_file_content(&config.file_path) {
        Ok(content) => content, // If successful, assign the file content
        Err(err) => match err {
            err => {
                println!("Error reading the file: {}", err);
                std::process::exit(1);
            }
        },
    };
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("File content: {:?}", file_content);
}
