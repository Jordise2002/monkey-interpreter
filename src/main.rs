use crate::token::Token::IDENTIFIER;

mod token;
mod lexer;
mod test_parser;
mod repl;
mod ast;
mod parser;
mod object;
mod test_evaluator;
mod evaluator;
mod environment;

fn main() {
    repl::start();
}
