use neotally::*;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let file_path: String = arguments[1].clone();
    
    println!(
        "{}",
        Tally::get_formatted_string(get_tally(file_path))
    );
}
