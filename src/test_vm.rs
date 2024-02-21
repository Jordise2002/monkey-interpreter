use std::collections::HashMap;
use std::vec;
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

#[test]
fn test_function_without_params()
{
    let tests = vec![
        VmTestCase
        {
            input: "let fivePlusTen = fn() { 5 + 10 }; fivePlusTen();".to_string(),
            expected: Object::IntegerObject(15)
        },
        VmTestCase {
            input: "let one = fn() { 1; };
                    let two = fn() { 2;  };
                    one() + two()".to_string(),
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "let a = fn() { 1 };
                    let b = fn() { a() + 1 };
                    let c = fn() { b() + 1 };
                    c();".to_string(),
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "let earlyExit  = fn() { return 99; 100; }; earlyExit();".to_string(),
            expected: Object::IntegerObject(99)
        }
    ];

    run_vm_tests(tests);
}
#[test]
fn test_function_without_return()
{   
    let tests = vec![
        VmTestCase {
            input: "let noReturn = fn() { };
                    noReturn();".to_string(),
            expected: Object::Null
        },
        VmTestCase {
            input: "let noReturn = fn() { };
                    let noReturnTwo = fn() { noReturn(); };
                    noReturnTwo();".to_string(),
            expected: Object::Null
        }
    ];

    run_vm_tests(tests);
}

#[test]
fn test_local_bindings() {
    let tests = vec![
        VmTestCase {
            input: "let one = fn() {let one = 1; one;};
            one();".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase {
            input: "let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };oneAndTwo();".to_string(),
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
                    let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
                    oneAndTwo() + threeAndFour()".to_string(),
            expected: Object::IntegerObject(10)
        },
    ];

    run_vm_tests(tests);
}

#[test]
fn test_calling_functions_with_args()
{
    let test = vec![
        VmTestCase{
            input: "let identity = fn(a){ a; };identity(4);".to_string(),
            expected: Object::IntegerObject(4)
        },
        VmTestCase {
            input: "let sum = fn(a, b) { a + b; }; sum(1, 2);".to_string(),
            expected: Object::IntegerObject(3)
        },
        VmTestCase {
            input: "let globalNum = 10;
                    let sum = fn(a, b) {
                        let c = a + b;
                        c + globalNum;
                    };
                    
                    let outer = fn() {
                        sum(1, 2) + sum(3, 4) + globalNum;
                    };
                    
                    outer() + globalNum;".to_string(),
            expected: Object::IntegerObject(50)
        }
    ];

    run_vm_tests(test);
}

#[test]
fn test_built_in_fn()
{
    let test = vec![
        VmTestCase
        {
            input: "len(\"\")".to_string(),
            expected: Object::IntegerObject(0)
        },
        VmTestCase 
        {
            input: "len(\"four\")".to_string(),
            expected: Object::IntegerObject(4)
        },
        VmTestCase
        {
            input: "len(\"hello world\")".to_string(),
            expected: Object::IntegerObject(11)
        },
        VmTestCase
        {
            input: "len(1)".to_string(),
            expected: Object::Error("not suported type: INTEGER".to_string())
        },
        VmTestCase
        {
            input: "len(\"one\", \"two\");".to_string(),
            expected: Object::Error("wrong number of arguments: got = 2, want = 1".to_string())
        },
        VmTestCase
        {
            input: "len([1, 2, 3]);".to_string(),
            expected:Object::IntegerObject(3)
        },
        VmTestCase
        {
            input: "len([]);".to_string(),
            expected: Object::IntegerObject(0)
        },
        VmTestCase
        {
            input: "first([1, 2]);".to_string(),
            expected: Object::IntegerObject(1)
        },
        VmTestCase
        {
            input: "push([], 1);".to_string(),
            expected: Object::Array(vec![Box::new(Object::IntegerObject(1))])
        }
    ];

    run_vm_tests(test);
}