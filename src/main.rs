mod tokens;

use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let mut is_error = false;
    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let token_iterator = tokens::TokenIterator::new(&file_contents);
            for token in token_iterator {
                match token {
                    Ok(tok) => println!("{}", tok.to_string()),
                    Err(e) => {
                        eprintln!("{}", e);
                        is_error = true;
                    }
                }
            }
            if is_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            exit(-1);
        }
    }
}
