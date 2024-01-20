use std::{fs, process};
use std::io::ErrorKind;

mod compiler;
mod parser;
mod arg_parser;

use arg_parser::read_args;

use crate::compiler::compile;
use crate::parser::parse; 


fn main() {
    // parse the arguments given from the command line: the input file and the output file
    let (input_file, output_file) = read_args();

    let contents = fs::read_to_string(&input_file);
    if let Err(contents) = contents {
        match contents.kind() {
            ErrorKind::NotFound => {
                println!("File {} was not found!", &input_file);
                process::exit(1);
            },
            _ => {
                println!("Error reading file {}!", &input_file);
                process::exit(1);
            }
        }
    }

    println!("Parsing program {}...", &input_file);

    // contents was verified to be String above. this is safe.
    let text = contents.unwrap();

    // now we use the parser on the text
    let ast = parse(&text);

    // now we compile
    println!("Compiling ...");
    compile(ast, &output_file);
}
