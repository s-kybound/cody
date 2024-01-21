mod syntax_tree {
    pub mod ast;
}

mod compiler {
    pub mod ir_constructor;
    pub mod scope;
}

mod parser {
    mod ast_generator;
    mod lexer;

    pub mod token_types;
}