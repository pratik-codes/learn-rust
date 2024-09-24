mod utils;

fn main() {
    println!("Mini grep by Pratik...");

    let args: Vec<String> = utils::get_command_line_args();
    let config = utils::parse_config(&args);
    let file_content: String = utils::read_file_content(&config.file_path);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("File content: {:?}", file_content);
}
