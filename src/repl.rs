use std::io::Write;
use crate::ast::Node;
use crate::compiler::Compiler;
use crate::environment::Environment;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::vm::Vm;

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

        /*let evaluated = eval(Node::Program(program),& mut env);
        if let Object::Null = &evaluated
        {
            continue;
        }
        println!("{}", evaluated.inspect());

        */

        let mut compiler = Compiler::new();
        compiler.compile(Node::Program(program));
        let mut vm = Vm::new(compiler.get_bytecode());
        vm.run();
        let value = vm.last_popped_stack_element();
        if let Object::Null = &value
        {
            continue;
        }
        println!("{}", value.inspect());

    }
}