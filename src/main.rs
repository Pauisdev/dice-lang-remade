use crate::{parser::Root, tokenizer::Tokenize};

mod args;
mod parser;
mod tokenizer;

fn main() -> Result<(), lexopt::Error> {
    let mut args = args::parse_args()?;
    args.file_text.retain(|c| c != '\n');
    let tokens = args.file_text.tokenize();
    println!("{:#?}", tokens);

    let root = Root {
        children: tokens.into(),
    };

    println!("{:#?}", root);

    Ok(())
}
