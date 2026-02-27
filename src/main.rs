mod tokens;

use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut token_iterator = tokens::TokenIterator::new(&file_contents);
            for token in token_iterator {
                println!("{}", token.to_string());
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
