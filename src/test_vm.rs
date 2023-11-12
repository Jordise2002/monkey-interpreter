use std::collections::HashMap;
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
        let program = parse(test.input.clone());

        let mut compiler = Compiler::new();
        compiler.compile(Node::Program(program));

        let mut vm = Vm::new(compiler.get_bytecode());
        vm.run();

        let stack_element = vm.last_popped_stack_element();

        assert_eq!(test.expected, stack_element, "{}", test.input);
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
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "1 * 2".to_string(),
            expected: Object::IntegerObject(2)
        },
        VmTestCase {
            input: "2 - 1".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase {
            input: "2 / 2".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase {
            input: "-1;".to_string(),
            expected: Object::IntegerObject(-1)
        },
        VmTestCase {
            input: "!!1".to_string(),
            expected: Object::BooleanObject(false)
        }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_boolean_arithmetic()
{
    let tests = vec![
        VmTestCase {
            input: "true;".to_string(),
            expected: Object::BooleanObject(true)
        },
        VmTestCase {
            input: "false".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 < 2".to_string(),
            expected: Object::BooleanObject(true)
        },
        VmTestCase {
            input: "1 > 2".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 < 1".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 > 1".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 == 1".to_string(),
            expected: Object::BooleanObject(true)
        },
        VmTestCase {
            input: "1 == 2".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 != 1".to_string(),
            expected: Object::BooleanObject(false)
        },
        VmTestCase {
            input: "1 != 2".to_string(),
            expected: Object::BooleanObject(true)
        }
    ];
    run_vm_tests(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
      VmTestCase {
          input: "if(true) { 10 }".to_string(),
          expected: Object::IntegerObject(10)
      },
      VmTestCase {
          input: "if(true) { 10 } else { 20 }".to_string(),
          expected: Object::IntegerObject(10)
      },
      VmTestCase {
          input: "if(false) { 10 } else { 20 }".to_string(),
          expected: Object::IntegerObject(20)
      },
      VmTestCase {
          input: "if(1) { 10 }".to_string(),
          expected: Object::IntegerObject(10)
      },
      VmTestCase {
          input: "if( 1 < 2 ) { 10 }".to_string(),
          expected: Object::IntegerObject(10)
      },
      VmTestCase{
          input: "if( 1 < 2) { 10 } else { 20 }".to_string(),
          expected: Object::IntegerObject(10)
      },
      VmTestCase{
          input: "if( 1 > 2) { 10 } else { 20 }".to_string(),
          expected: Object::IntegerObject(20)
      },
      VmTestCase {
          input: "if ( 1 > 2) { 10 }".to_string(),
          expected: Object::Null
      },
      VmTestCase {
          input: "if (false) { 10 }".to_string(),
          expected: Object::Null
      }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_global_let_statements() {
    let tests = vec![
        VmTestCase{input: "let one = 1; one;".to_string(), expected: Object::IntegerObject(1)},
        VmTestCase{input: "let one = 1; let two = 2; one + two;".to_string(), expected: Object::IntegerObject(3)},
        VmTestCase{input: "let one = 1; let two = one + one; one + two".to_string(), expected: Object::IntegerObject(3)}
    ];

    run_vm_tests(tests);
}

#[test]
fn test_string_expressions()
{
    let tests = vec![
        VmTestCase{
            input: "\"monkey\"".to_string(),
            expected: Object::StringObject("monkey".to_string())
        },
        VmTestCase {
            input: "\"mon\" + \"key\"".to_string(),
            expected: Object::StringObject("monkey".to_string())
        },
        VmTestCase {
            input:"\"monkey\" + \"banana\"".to_string(),
            expected: Object::StringObject("monkeybanana".to_string())
        }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_array_expr() {
    let tests = vec![
        VmTestCase{
            input: "[]".to_string(),
            expected:Object::Array(vec![])
        },
        VmTestCase{
            input: "[1, 2, 3];".to_string(),
            expected:Object::Array(vec![
                Box::new(Object::IntegerObject(1)),
                Box::new(Object::IntegerObject(2)),
                Box::new(Object::IntegerObject(3))])
        }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_hash_expr() {
    let tests = vec![
        VmTestCase{
            input: "{}".to_string(),
            expected: Object::HashMap(HashMap::new())
        },
        VmTestCase {
            input: "{1:2, 3:4}".to_string(),
            expected: Object::HashMap(HashMap::from([(Object::IntegerObject(1),Object::IntegerObject(2)), (Object::IntegerObject(3), Object::IntegerObject(4))]))
        }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_index_expression() {
    let tests = vec![
        VmTestCase {
            input: "[1, 2, 3][1]".to_string(),
            expected: Object::IntegerObject(2)
        },
        VmTestCase {
            input: "[1, 2, 3][0 + 2]".to_string(),
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "[[1, 1, 1]][0][0]".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase {
            input: "[][0]".to_string(),
            expected: Object::Null
        },
        VmTestCase {
            input: "[1, 2, 3][99]".to_string(),
            expected: Object::Null
        },
        VmTestCase {
            input: "{1: 1, 2: 2}[1]".to_string(),
            expected: Object::IntegerObject(1)
        }
    ];

    run_vm_tests(tests);
}