use std::{iter::Peekable, vec::IntoIter};

use crate::tokenizer::Token;

pub struct Root {
    pub children: Vec<Node>,
}

#[derive(Debug)]
enum ReservedFunction {
    Input(String),
    Sleep(i32),
}

#[derive(Debug)]
enum ValueType {
    Str(String),
    Num(i32),
}

#[derive(Debug)]
pub enum Node {
    FnCall { kind: ReservedFunction },
    Loop { children: Vec<Node> },
    Increment { who: String, amount: i32 },
    PushStr { who: String, value: String },
    DefineVariable { name: String, value: ValueType },
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = vec![];
    let mut iter = tokens.into_iter().peekable();

    loop {
        let token = iter.next();
        if token.is_none() {
            break;
        }
        let token = token.unwrap();

        match token {
            Token::Let => {
                let identifier = iter.next().unwrap();
                if let Token::Identifier(id) = identifier {
                    assert!(iter.next().unwrap() == Token::Equals);
                    match iter.next().unwrap() {
                        Token::Num(num) => nodes.push(Node::DefineVariable {
                            name: id,
                            value: ValueType::Num(num),
                        }),
                        Token::Str(val) => nodes.push(Node::DefineVariable {
                            name: id,
                            value: ValueType::Str(val),
                        }),
                        _ => panic!("Expected to find Num or Str following Equals"),
                    }
                } else {
                    panic!("Expected to find Identifier following Let.")
                }
            }
            Token::Identifier(id)
                if iter.peek().is_some_and(|t| *t == Token::Plus) && {
                    iter.next();
                    iter.peek().is_some_and(|t| *t == Token::Equals)
                } =>
            {
                iter.next();
                match iter.next().unwrap() {
                    Token::Num(num) => nodes.push(Node::Increment {
                        who: id,
                        amount: num,
                    }),
                    Token::Str(value) => nodes.push(Node::PushStr { who: id, value }),
                    _ => panic!("Expected to find Num or Str following Equals"),
                }
            }
            Token::Loop => {
                assert!(iter.next().unwrap() == Token::OpenBracket);
                let tokens = read_till(&mut iter, Token::CloseBracket);
                let children = parse(tokens);
                nodes.push(Node::Loop { children });
            }
            _ => {}
        }
    }

    nodes
}

fn read_till(from: &mut Peekable<IntoIter<Token>>, end: Token) -> Vec<Token> {
    let mut tokens = vec![];
    loop {
        let token = from.next();
        if token.is_none() {
            break;
        }
        let token = token.unwrap();
        if token == end {
            break;
        }
        tokens.push(token);
    }
    tokens
}
