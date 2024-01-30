use std::env::args;

use ansi_converter::string_to_ansi_16;

fn main() {
    let input = args().nth(1).expect("Please provide an ANSI file to read.");

    let content = std::fs::read_to_string(input).expect("Failed to read file.");

    let output = string_to_ansi_16(&content);

    println!("{output}");
}
