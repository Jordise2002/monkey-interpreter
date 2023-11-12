
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
#[cfg(test)]
mod test_code;
mod compiler;
#[cfg(test)]
mod test_compiler;
#[cfg(test)]
mod test_vm;
mod vm;
mod symbol_table;
#[cfg(test)]
mod test_symbol_table;

fn main() {
    repl::start();
}
