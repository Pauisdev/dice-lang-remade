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
