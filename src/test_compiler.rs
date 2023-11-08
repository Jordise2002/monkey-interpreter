use crate::ast::{Node, Program};
use crate::code::{Instructions, make};
use crate::code::Opcode::{OpAdd, OpBang, OpConstant, OpDiv, OpEq, OpGreaterThan, OpJump, OpJumpNotTrue, OpMinus, OpMul, OpNotEq, OpNull, OpPop, OpSub, OpTrue};
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

struct CompilerTestCase {
    input: String,
    expected_constants: Vec<Object>,
    expected_instructions: Vec<Instructions>
}

fn parse(input: String) -> Program
{
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

fn run_compiler_tests(tests: Vec<CompilerTestCase>) {
    for test in tests {
        let program = parse(test.input.clone());
        let mut compiler = Compiler::new();
        compiler.compile(Node::Program(program));
        let bytecode = compiler.get_bytecode();
        test_instructions(bytecode.instructions, test.expected_instructions);
        test_constants(bytecode.constants, test.expected_constants);
    }
}

pub fn concat_instructions(instructions_vec: Vec<Instructions>) -> Instructions
{
    let mut output = Vec::new();
    for mut instruction in instructions_vec {
        output.append(& mut instruction.content);
    }
    Instructions{content:output}
}

fn test_instructions(actual: Instructions, expected: Vec<Instructions>) {
    let expected = concat_instructions(expected.clone());
    assert_eq!(actual.content.len(), expected.content.len(), "wrong length got {} wanted {}", actual, expected);
    for (actual_byte, expected_byte) in actual.content.clone().into_iter().zip(expected.content.clone())
    {
        assert_eq!(actual_byte, expected_byte, "wrong byte got {} wanted {}", actual, expected);
    }
}

fn test_constants(actual: Vec<Object>, expected: Vec<Object>)
{
    for (actual, expected) in actual.into_iter().zip(expected)
    {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        CompilerTestCase {
            input: "1 + 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpAdd, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 - 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpSub, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 * 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpMul, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 / 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpDiv, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "-1;".to_string(),
            expected_constants: vec![Object::IntegerObject(1)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpMinus, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "!!1;".to_string(),
            expected_constants: vec![Object::IntegerObject(1)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpBang, vec![]).unwrap(),
                                        make(OpBang, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        }
    ];
    run_compiler_tests(tests);
}

#[test]
fn test_boolean_arithmetic()
{
    let tests = vec![
        CompilerTestCase {
            input: "true;".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(OpTrue, vec![]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "1 > 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpGreaterThan, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 < 2".to_string(),
            expected_constants: vec![Object::IntegerObject(2), Object::IntegerObject(1)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpGreaterThan, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 == 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpEq, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
        CompilerTestCase {
            input: "1 != 2".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![make(OpConstant, vec![0]).unwrap(),
                                        make(OpConstant, vec![1]).unwrap(),
                                        make(OpNotEq, vec![]).unwrap(),
                                        make(OpPop, vec![]).unwrap()]
        },
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
        CompilerTestCase {
            input: "if(true) { 10}; 3333;".to_string(),
            expected_constants: vec![Object::IntegerObject(10), Object::IntegerObject(3333)],
            expected_instructions: vec![
                make(OpTrue, vec![]).unwrap(),
                make(OpJumpNotTrue, vec![10]).unwrap(),
                make(OpConstant, vec![0]).unwrap(),
                make(OpJump, vec![11]).unwrap(),
                make(OpNull, vec![]).unwrap(),
                make(OpPop, vec![]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "if(true) { 10 } else { 20 }; 3333;".to_string(),
            expected_constants: vec![Object::IntegerObject(10), Object::IntegerObject(20), Object::IntegerObject(3333)],
            expected_instructions: vec![
                make(OpTrue, vec![]).unwrap(), //0000
                make(OpJumpNotTrue, vec![10]).unwrap(), //0001
                make(OpConstant, vec![0]).unwrap(), //0004
                make(OpJump, vec![13]).unwrap(), //0007
                make(OpConstant, vec![1]).unwrap(), //0010
                make(OpPop, vec![]).unwrap(), //0013
                make(OpConstant, vec![2]).unwrap(), //0014
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];
    run_compiler_tests(tests);
}
