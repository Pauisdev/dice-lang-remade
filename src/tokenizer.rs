pub fn tokenize(file_text: String) -> Vec<Token> {
    let mut stack = String::new();
    let mut tokens = vec![];
    let mut quotes_open = false;
    for i in 0..file_text.len() - 1 {
        let bytes = file_text.as_bytes();
        let char = bytes[i] as char;
        let next_char = if i + 1 > file_text.len() - 1 {
            None
        } else {
            let char = bytes[i + 1] as char;
            Some(char)
        };
        if char == ' ' && !quotes_open {
            continue;
        }
        if next_char.is_none() || next_char.unwrap() == ' ' {
            let token = match stack.as_str() {
                "let" => Some(Token::Let),
                "loop" => Some(Token::Loop),
                _ => None,
            };
            if let Some(token) = token {
                tokens.push(token);
                stack.clear();
            }
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
                    tokens.push(Token::Str(stack.clone()));
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
                Ok(num) => Token::Num(num),
                Err(_) => Token::Identifier(value),
            },
            _ => token,
        })
        .collect();
    tokens
}
#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Loop,
    Identifier(String),
    Num(i32),
    Str(String),
    Equals,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Plus,
    OpenBracket,
    CloseBracket,
    QuotationMark,
}

pub fn generate_tree(tokens: Vec<Token>) {
    println!("Generating tree below...");
    for (i, token) in tokens.iter().enumerate() {
        println!("{} {:?}", "-".repeat(i + 1), token);
    }
    println!(
        "Tree generated! Visit https://vgarciasc.github.io/tree-renderer/ and paste it to view it."
    )
}
