
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
mod code;
mod test_code;

fn main() {
    repl::start();
}
