use core::panic;
use std::{env::args, fs};

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

    let mut stack = String::new();
    let mut tokens = vec![];
    let mut quotes_open = false;
    for char in file_text.chars() {
        if char == ' ' && !quotes_open {
            continue;
        }
        let token = match stack.as_str() {
            "let" => Some(Token::Let),
            "loop" => Some(Token::Loop),
            _ => None,
        };
        if let Some(token) = token {
            tokens.push(token);
            stack.clear();
        }
        let token = match char {
            '=' => Token::Equals,
            ';' => Token::Semicolon,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '{' => Token::OpenBracket,
            '}' => Token::CloseBracket,
            '+' => Token::Plus,
            '"' => {
                quotes_open = !quotes_open;
                if !quotes_open {
                    tokens.push(Token::Text(stack.clone()));
                    stack.clear();
                }
                Token::QuotationMark
            }
            _ => {
                stack.push(char);
                continue;
            }
        };
        if !stack.is_empty() {
            tokens.push(Token::Identifier(stack.clone()));
            stack.clear();
        }
        tokens.push(token);
    }
    let tokens: Vec<Token> = tokens
        .into_iter()
        .map(|token| match token {
            Token::Identifier(value) => match value.parse::<i32>() {
                Ok(num) => Token::Number(num),
                Err(_) => Token::Identifier(value),
            },
            _ => token,
        })
        .collect();
    println!("{:?}", tokens);
}

#[derive(Debug)]
enum Token {
    Let,
    Loop,
    Identifier(String),
    Number(i32),
    Text(String),
    Equals,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Plus,
    OpenBracket,
    CloseBracket,
    QuotationMark,
}
