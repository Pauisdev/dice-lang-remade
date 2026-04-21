use colored::Colorize;
use std::fs;

pub struct Args {
    pub file_text: String,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file_text = None;
    let mut parser = lexopt::Parser::from_env();

    let version = (String::from("v") + env!("CARGO_PKG_VERSION"))
        .cyan()
        .bold();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('v') | Long("version") => {
                println!("Currently installed version of dice-lang: {}", version);
                std::process::exit(0);
            }
            Short('r') | Long("run") => {
                file_text = match fs::read_to_string(parser.value()?.to_str().unwrap()) {
                    Ok(text) => Some(text),
                    Err(err) => panic!("Failed during file read: {}", err),
                };
            }
            Short('h') | Long("help") => {
                println!("{}", "🎲 Dice lang's compiler 🎲".green().bold());
                println!(
                    "{} {} {}",
                    "Usage:".green().bold(),
                    "dice".cyan().bold(),
                    "[OPTIONS] [COMMAND]".blue()
                );
                println!();
                println!("{}", "Options:".green().bold());
                println!(
                    "  {}, {}{: >29}",
                    "-v".cyan().bold(),
                    "--version".cyan().bold(),
                    "Print current version"
                );
                println!(
                    "  {}, {}{: >24}",
                    "-h".cyan().bold(),
                    "--help".cyan().bold(),
                    "See this menu"
                );
                println!();
                println!("{}", "Commands:".green().bold());
                println!(
                    "  {}, {}{: >27}",
                    "build".cyan().bold(),
                    "b".cyan().bold(),
                    "Compile a file"
                );
                println!(
                    "  {}, {}{: >25}",
                    "run".cyan().bold(),
                    "r".cyan().bold(),
                    "Run a file"
                );
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        file_text: file_text.ok_or("Missing file path")?,
    })
}
