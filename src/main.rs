use core::panic;
use std::{env::args, fs};

fn main() {
    let file_text = match args().nth(1) {
        Some(file_path) => match fs::read_to_string(file_path) {
            Ok(text) => text,
            Err(err) => {
                panic!("Failed during file read: {}", err);
            }
        },
        None => {
            panic!("Missing file path")
        }
    };
    for char in file_text.chars() {
        println!("{}", char);
    }
}

enum Token {
    LET,
    TEXT,
    EQUALS,
    OPEN_PARENTHESIS,
    CLOSE_PARENTHESIS,
    SEMICOLON,
    PLUS,
    OPEN_BRACKET,
    CLOSE_BACKET,
}

struct Root {
    tokens: Vec<Token>,
}
