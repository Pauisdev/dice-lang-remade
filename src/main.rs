use crate::tokenizer::generate_tree;
mod args;
mod parser;
mod tokenizer;

fn main() -> Result<(), lexopt::Error> {
    let mut args = args::parse_args()?;
    args.file_text.retain(|c| c != '\n');
    let tokens = tokenizer::tokenize(args.file_text);
    if args.tree_view {
        generate_tree(tokens);
    }

    Ok(())
}
