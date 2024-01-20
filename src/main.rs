use std::{fs, process};
use std::io::ErrorKind;

use clap::Parser;

mod parser;
use crate::parser::lexer::lex;

#[derive(Parser)]
#[command(author = "s-kybound")] 
#[command(version = "0.0.1")]
#[command(about = "Cody language compiler", long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input_file: String,

    #[arg(default_value = "a.out")]
    #[arg(short = 'o', long = "output")]
    output_file: String,
}
fn main() {
    let cli_args = Args::parse();

    let contents = fs::read_to_string(&cli_args.input_file);
    if let Err(contents) = contents {
        match contents.kind() {
            ErrorKind::NotFound => {
                println!("File {} was not found!", cli_args.input_file);
                process::exit(1);
            },
            _ => {
                println!("Error reading file {}!", cli_args.input_file);
                process::exit(1);
            }
        }
    }

    println!("Compiling file {}...", cli_args.input_file);

    // contents was verified to be String above. this is safe.
    let text = contents.unwrap();

    // now we use the lexer on the text
    let tokens = lex(&text);
    for token in tokens {
        println!("{:?}", token);
    }
}
