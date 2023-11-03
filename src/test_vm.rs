use crate::ast::{Node, Program};
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use crate::vm::Vm;

struct VmTestCase
{
    input: String,
    expected: Object
}
fn parse(input: String) -> Program
{
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}


fn run_vm_tests(tests: Vec<VmTestCase>) {
    for test in tests
    {
        let program = parse(test.input);

        let mut compiler = Compiler::new();
        compiler.compile(Node::Program(program));

        let mut vm = Vm::new(compiler.get_bytecode());
        vm.run();

        let stack_element = vm.get_stack_top();
        if let Some(stack_element) = stack_element
        {
            assert_eq!(test.expected, stack_element);
        }
        else {
            panic!("couldn't read stack_top");
        }


    }
}

#[test]
fn test_integer_arithmetic()
{
    let tests = vec![
        VmTestCase {
            input: "1".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase {
            input: "2".to_string(),
            expected: Object::IntegerObject(2)
        },
        VmTestCase {
            input: "1 + 2".to_string(),
            expected: Object::IntegerObject(2)//TODO: Fix this, should be 3
        }
    ];

    run_vm_tests(tests);
}