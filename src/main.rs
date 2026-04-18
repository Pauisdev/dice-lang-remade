use core::panic;
use std::fs;

use crate::tokenizer::generate_tree;
mod tokenizer;

struct Args {
    file_text: String,
    tree_view: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file_text = None;
    let mut tree_view = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("tree-view") => tree_view = true,
            Value(val) if file_text.is_none() => {
                file_text = match fs::read_to_string(val.string()?) {
                    Ok(text) => Some(text),
                    Err(err) => panic!("Failed during file read: {}", err),
                };
            }
            Long("help") => {
                println!("Usage: dice [--tree-view] file");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        file_text: file_text.ok_or("Missing file path")?,
        tree_view,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let mut args = parse_args()?;
    args.file_text.retain(|c| c != '\n');
    let tokens = tokenizer::tokenize(args.file_text);
    if args.tree_view {
        generate_tree(tokens);
    }

    Ok(())
}
