//! SUPER simple lexer.
//! Lexes the program into tokens that the parser can work on.

use crate::parser::token_types::{Token, AtomBinary};

// #[macro_export]
// macro_rules! remove_from_program {
//     ( $prog:expr ,$( $x:expr ),* ) => {
//         {
//             let mut temp = $prog.clone()
//             $(
//                 .replace($x, format!(" {} ", &$x));
//             )*
//             temp
//         }
//     };
// }
/// Lexes a program string into an array of Tokens.
pub fn lex(program: &str) -> Vec<Token> {
    // let formatted_program = remove_from_program!(program, "(", ")", "[", "]", ".", "'", "`", "@", ";", "|", "->", "$+", "$-", "$*", "$/", "$=", "$<", "$!", "$&", "$|");
    let formatted_program = program
    .replace("(", " ( ")
    .replace(")", " ) ")
    .replace("[", " [ ")
    .replace("]", " ] ")
    .replace(".", " . ")
    .replace("'", " ' ")
    .replace("`", " ` ")
    .replace("@", " @ ")
    .replace(";", " ; ")
    .replace("|", " | ")
    .replace("->", " -> ")
    
    // atomic binary operators that we will use to bulld the basic library
    .replace("$+", " $+ ")
    .replace("$-", " $- ")
    .replace("$*", " $* ")
    .replace("$/", " $/ ")
    .replace("$=", " $= ")
    .replace("$<", " $< ")
    .replace("$!", " $! ")
    .replace("$&", " $& ")
    .replace("$|", " $| ");
    
    let mut tokens: Vec<Token> = Vec::new();
    // the entire program is treated as a sequence expression, 
    // hence we prepare the token stream to be parsed as such
    tokens.push(Token::LeftPar);
    tokens.push(Token::Seq);
    for curr_line in formatted_program.lines() {
        for curr_word in curr_line.split_whitespace() {
            match curr_word {
                // comments
                ";" => break,

                // parantheses
                "(" => tokens.push(Token::LeftPar),
                ")" => tokens.push(Token::RightPar),

                // match case syntaxfrom
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
                
                // continuations
                "cont" => tokens.push(Token::Cont),

                // pair syntax
                "[" => tokens.push(Token::LeftBkt),
                "]" => tokens.push(Token::RightBkt),
                "." => tokens.push(Token::Dot),

                // quote syntax
                "`" => tokens.push(Token::Grave),
                "'" => tokens.push(Token::Quote),
                "@" => tokens.push(Token::At),

                // external functions
                "extern" => tokens.push(Token::Extern),

                // atomic binary operators
                "$+" => tokens.push(Token::AtomicOp(AtomBinary::Add)),
                "$-" => tokens.push(Token::AtomicOp(AtomBinary::Sub)),
                "$*" => tokens.push(Token::AtomicOp(AtomBinary::Mul)),
                "$/" => tokens.push(Token::AtomicOp(AtomBinary::Div)),
                "$=" => tokens.push(Token::AtomicOp(AtomBinary::Eq)),
                "$<" => tokens.push(Token::AtomicOp(AtomBinary::Lt)),
                "$!" => tokens.push(Token::AtomicOp(AtomBinary::Not)),
                "$&" => tokens.push(Token::AtomicOp(AtomBinary::And)),
                "$|" => tokens.push(Token::AtomicOp(AtomBinary::Or)),

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
    // add the ending of the sequence expression
    tokens.push(Token::RightPar);

    // add the last token
    tokens.push(Token::EOF);
    tokens
}