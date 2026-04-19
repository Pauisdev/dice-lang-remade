#[derive(Debug, PartialEq, Clone)]
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
}

pub trait Tokenize {
    fn tokenize(self) -> Tokens;
}

impl Tokenize for String {
    fn tokenize(self) -> Tokens {
        let mut stack = String::new();
        let mut tokens = vec![];
        let mut quotes_open = false;
        for i in 0..self.len() - 1 {
            let bytes = self.as_bytes();
            let char = bytes[i] as char;
            let next_char = if i + 1 > self.len() - 1 {
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
                    continue;
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
        Tokens(tokens)
    }
}

#[derive(Clone)]
pub struct Tokens(pub Vec<Token>);

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: Vec<String> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, token)| format!("{} {:?}", "-".repeat(i + 1), token))
            .collect();
        let output = output.join("\n");
        write!(f, "{}", output)
    }
}
