use std::io::Write;
use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::token::Token;

const PROMPT: &str = ">>";
pub fn start() {
    let mut line = String::new();
    let mut env = Environment::new();
    loop{
        line.clear();
        print!("{}", PROMPT);
        std::io::stdout().flush().expect("Error while printing");
        let read_len = std::io::stdin().read_line(& mut line);
        if let Err(content) = read_len
        {
            println!("Error while reading, details: {}", content);
            return;
        }
        let lexer = Lexer::new(line.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if parser.errors.len() > 0 {
            for error in parser.errors
            {
                println!("{}", error);
            }
            continue;
        }

        let evaluated = eval(Node::Program(program),& mut env);
        if let Some(evl) = evaluated {
            if let Object::Null = &evl
            {
                continue;
            }
            println!("{}", evl.inspect());
        }

    }
}