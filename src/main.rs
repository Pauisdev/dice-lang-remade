use crate::{parser::Root, tokenizer::Tokenize};

mod args;
mod parser;
mod tokenizer;

fn main() -> Result<(), lexopt::Error> {
    let mut args = args::parse_args()?;
    args.file_text.retain(|c| c != '\n');
    let tokens = args.file_text.tokenize();

    let root = Root {
        children: tokens.clone().into(),
    };
    println!("{:#?}", root);

    if args.tree_view {
        println!("Generating tree below...");
        println!("{}", tokens);
        println!(
            "Tree generated! Visit https://vgarciasc.github.io/tree-renderer/ and paste it to view it."
        )
    }

    Ok(())
}
