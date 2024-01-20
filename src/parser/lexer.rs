//! SUPER simple lexer.
//! Lexes the program into tokens that the parser can work on.

use crate::parser::token_types::Token;

/// Lexes a program string into an array of Tokens.
pub fn lex(program: &str) -> Vec<Token> {
    let formatted_program = program
    .replace("(", " ( ")
    .replace(")", " ) ")
    .replace(";", " ; ")
    .replace("|", " | ")
    .replace("->", " -> ");
    
    let mut tokens: Vec<Token> = Vec::new();
    for curr_line in formatted_program.lines() {
        for curr_word in curr_line.split_whitespace() {
            match curr_word {
                // comments
                ";" => break,

                // parantheses
                "(" => tokens.push(Token::LeftPar),
                ")" => tokens.push(Token::RightPar),

                // match case syntax
                "|" => tokens.push(Token::Pipe),
                "->" => tokens.push(Token::Arrow),
                "match" => tokens.push(Token::Match),

                // sequence expressions
                "seq" => tokens.push(Token::Seq),

                // definition syntax
                "define" => tokens.push(Token::Define),

                // functions
                "fn" => tokens.push(Token::Function),
                
                // conditionals
                "if" => tokens.push(Token::If),
                "else" => tokens.push(Token::Else),
                
                // continuations
                "cont" => tokens.push(Token::Cont),

                // integers and identifiers
                rest => {
                    if rest.parse::<i32>().is_ok() {
                        tokens.push(Token::Integer(rest.parse::<i32>().unwrap()));
                    } else {
                        tokens.push(Token::Identifier(curr_word.to_string()));
                    }
                },
            }
        }
    }
    tokens
}