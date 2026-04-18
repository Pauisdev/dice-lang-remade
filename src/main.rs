use core::panic;
use std::{env::args, fs};
mod tokenizer;

fn main() {
    let mut file_text = match args().nth(1) {
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
    file_text.retain(|c| c != '\n');
    let tokens = tokenizer::tokenize(file_text);
    println!("{:?}", tokens);
}
