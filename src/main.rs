extern crate neotally;
use std::env;

use neotally::Tally;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let file_path: String = arguments[1].clone();
    
    println!(
        "{}",
        Tally::get_formatted_string(Tally::new().calculate(file_path))
    );
}
