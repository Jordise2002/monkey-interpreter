
mod token;
mod lexer;
#[cfg(test)]
mod test_parser;
mod repl;
mod ast;
mod parser;
mod object;
#[cfg(test)]
mod test_evaluator;
mod evaluator;
mod environment;
mod builtins;

fn main() {
    repl::start();
}
