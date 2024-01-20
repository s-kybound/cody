use crate::parser::token_types::Token::*;
use crate::parser::token_types::Token; 
use crate::parser::node_types::ExpressionAST;
use crate::parser::node_types::ExpressionAST::*;

/// This module is responsible for generating the AST from the tokens.
pub fn ast_generate(tokens: &Vec<Token>) -> ExpressionAST {
    // we treat the token stream as a stack, reversing it to pop from the front
    let mut remaining_tokens = tokens.clone();
    remaining_tokens.reverse();

    parse(&mut remaining_tokens)
}

/// Parses the token stream into an AST.
fn parse(tokens: &mut Vec<Token>) -> ExpressionAST {
    match tokens.pop() {
        Some(token) => match token {
            // if the EOF is reached, return the NoneExpr.
            // if it's in an internal recursive parse call the
            // outer parse call will throw an error anyway.
            EOF => NoneExpr,
            LeftPar => parse_grouping(tokens),
            LeftBkt => parse_pair(tokens), 
            // Grave => parse_quote(tokens, Grave),
            // Quote => parse_quote(tokens, Quote),
            Integer(i) => IntegerExpr(i),
            Identifier(s) => VariableExpr(s),

            // everything else met at this level is an error
            _ => panic!("Unexpected token: {:?}", token),


        },
        None => panic!("Unexpected end of token stream."),
    }
}

fn parse_grouping(tokens: &mut Vec<Token>) -> ExpressionAST {
    let curr_token = tokens.pop().expect("Unexpected end of token stream.");
    match curr_token {
        RightPar => NoneExpr,

        // function objects
        Function => parse_function(tokens),

        // sequence expressions
        Seq => parse_sequence(tokens),

        // definitions
        Define => parse_definition(tokens),

        // conditionals
        If => parse_conditional(tokens),

        // match case
        Match => parse_match(tokens),

        // // continuations
        // Cont => parse_continuation(tokens),

        // // external functions
        // Extern => parse_extern(tokens),

        // identifiers or inner groupings
        Identifier(_) | LeftPar => {
            // add the token back so that we can evaluate it with parse()
            tokens.push(curr_token);
            parse_call(tokens)
        },

        // everything else is an error
        _ => panic!("Unexpected token: {:?}", curr_token),
    }
}

fn close_grouping(tokens: &mut Vec<Token>, final_expression: ExpressionAST) -> ExpressionAST{
    match tokens.pop() {
        Some(token) => match token {
            RightPar => final_expression,
            _ => panic!("Unexpected token: {:?}", token),
        },
        None => panic!("Unexpected end of token stream."),
    }
}

fn parse_function(tokens: &mut Vec<Token>) -> ExpressionAST {
    let mut parameters: Vec<ExpressionAST> = Vec::new();

    // parse the parameter bracket
    let mut curr_token = tokens.pop().expect("Unexpected end of token stream.");
    match curr_token {
        LeftPar => (),
        _ => panic!("Unexpected token: {:?}", curr_token),
    }

    // parse the parameters
    loop {
        curr_token = tokens.pop().expect("Unexpected end of token stream.");
        match curr_token {
            RightPar => break,
            Identifier(s) => {
                parameters.push(VariableExpr(s));
            },
            _ => panic!("Unexpected token: {:?}", curr_token),
        }
    }

    // parse the expression
    let function_expression = parse(tokens);

    let new_function = FunctionExpr(parameters, Box::new(function_expression));

    close_grouping(tokens, new_function)
}

fn parse_sequence(tokens: &mut Vec<Token>) -> ExpressionAST {
    let mut expressions: Vec<ExpressionAST> = Vec::new();
    // parse the expressions
    loop {
        let curr_token = tokens.pop().expect("Unexpected end of token stream.");
        match curr_token {
            RightPar => break,
            _ => {
                // add the token back so that we can evaluate it with parse()
                tokens.push(curr_token);
                
                expressions.push(parse(tokens));
            },
        }
    }
    SeqExpr(expressions)
}

fn parse_definition(tokens: &mut Vec<Token>) -> ExpressionAST {
    let identifier = tokens.pop().expect("Unexpected end of token stream.");
    let definition_node = match identifier {
        Identifier(s) => DefineExpr(Box::new(VariableExpr(s)), Box::new(parse(tokens))),
        _ => panic!("Unexpected token: {:?}", identifier),
    };

    close_grouping(tokens, definition_node)
}

fn parse_conditional(tokens: &mut Vec<Token>) -> ExpressionAST {
    let predicate = parse(tokens);
    let con = parse(tokens);
    let alt = parse(tokens);

    close_grouping(tokens, IfExpr(Box::new(predicate), Box::new(con), Box::new(alt)))
}

fn parse_match(tokens: &mut Vec<Token>) -> ExpressionAST {
    let expression = parse(tokens);
    let mut match_arms: Vec<ExpressionAST> = Vec::new();
    
    // parse the match arms
    loop {
        let mut curr_token = tokens.pop().expect("Unexpected end of token stream.");
        match curr_token {
            RightPar => break,
            Pipe => {
                let mut patterns: Vec<ExpressionAST> = Vec::new();
                loop {
                    curr_token = tokens.pop().expect("Unexpected end of token stream.");
                    match curr_token {
                        Arrow => break,
                        Integer(i) => patterns.push(IntegerExpr(i)),
                        // allowed for the catch-all case
                        Identifier(s) => patterns.push(VariableExpr(s)),
                        // we disallow matching on non-atomic data types
                        _ => panic!("Unexpected token: {:?}", curr_token),
                    }
                }
                let match_expression = parse(tokens);
                match_arms.push(MatchArmExpr(patterns, Box::new(match_expression)));
            },
            _ => panic!("Unexpected token: {:?}", curr_token),
        }
    }
    MatchExpr(Box::new(expression), match_arms)
}

// fn parse_continuation(tokens: &mut Vec<Token>) -> ExpressionAST {
//     let continuation_expression = parse(tokens);

//     close_grouping(tokens, ContExpr(Box::new(continuation_expression)))
// }

// fn parse_extern(tokens: &mut Vec<Token>) -> ExpressionAST {
//     let identifier = tokens.pop().expect("Unexpected end of token stream.");
//     let extern_node = match identifier {
//         Identifier(s) => ExternExpr(Box::new(VariableExpr(s))),
//         _ => panic!("Unexpected token: {:?}", identifier),
//     };

//     close_grouping(tokens, extern_node)
// }

fn parse_call(tokens: &mut Vec<Token>) -> ExpressionAST {
    let mut arguments: Vec<ExpressionAST> = Vec::new();
    let mut function = parse(tokens);

    // parse the arguments
    loop {
        let curr_token = tokens.pop().expect("Unexpected end of token stream.");
        match curr_token {
            RightPar => break,
            _ => {
                // add the token back so that we can evaluate it with parse()
                tokens.push(curr_token);
                
                arguments.push(parse(tokens));
            }, 
        }
    }

    CallExpr(Box::new(function), arguments)
}

fn parse_pair(tokens: &mut Vec<Token>) -> ExpressionAST {
    let head = parse(tokens);
    match tokens.pop() {
        Some(token) => match token {
            Dot => {
                let tail = parse(tokens);
                PairExpr(Box::new(head), Box::new(tail))
            },
            _ => panic!("Unexpected token: {:?}", token),
        },
        None => panic!("Unexpected end of token stream."),
    }
}