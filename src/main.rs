mod tokens;
mod parser;
mod syntax;
mod tokenizer;
mod evaluate;

use std::env;
use std::fs;
use std::process::exit;
use tokens::Token;
use tokenizer::TokenIterator;
use evaluate::Evaluate;

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

            let token_iterator = TokenIterator::new(&file_contents);
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
                exit(65);
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                exit(-1)
            });

            let token_iterator = TokenIterator::new(&file_contents);
            let tokens: Vec<Token> = token_iterator
                .map(|res| res.unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    exit(65);
                }))
                .collect();

            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => println!("{}", ast),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                exit(-1)
            });

            let token_iterator = TokenIterator::new(&file_contents);
            let tokens: Vec<Token> = token_iterator
                .map(|res| res.unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    exit(70);
                }))
                .collect();

            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => match ast.evaluate() {
                    Ok(value) => println!("{}", value),
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(70);
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    exit(70);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            exit(-1);
        }
    }
}
