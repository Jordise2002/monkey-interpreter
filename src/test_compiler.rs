use std::vec;

use crate::ast::{Node, Program};
use crate::code::{self, join_instructions, make, Instructions};
use crate::code::Opcode::{OpAdd, OpArray, OpBang, OpConstant, OpDiv, OpEq, OpGreaterThan, OpHash, OpIndex, OpJump, OpJumpNotTrue, OpMinus, OpMul, OpNotEq, OpNull, OpPop, OpSetGlobal, OpSub, OpTrue, OpCall};
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::{CompiledFunctionStruct, Object};
use crate::object::Object::IntegerObject;
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

#[test]
fn test_variables() {
    let tests = vec![
        CompilerTestCase {
            input: "let x = 1; let y = 2;".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2)],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpSetGlobal, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpSetGlobal, vec![1]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_string_expressions() {
    let tests = vec![
        CompilerTestCase {
            input: "\"monkey\"".to_string(),
            expected_constants:vec![Object::StringObject("monkey".to_string())],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "\"mon\" + \"key\"".to_string(),
            expected_constants: vec![Object::StringObject("mon".to_string()), Object::StringObject("key".to_string())],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpAdd, vec![]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_array_expr() {
    let tests = vec![
        CompilerTestCase {
            input:"[]".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(OpArray, vec![0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase{
            input: "[1, 2, 3];".to_string(),
            expected_constants: vec![Object::IntegerObject(1), Object::IntegerObject(2), Object::IntegerObject(3)],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpConstant, vec![2]).unwrap(),
                make(OpArray, vec![3]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "[1 + 2, 3 - 4, 5 * 6];".to_string(),
            expected_constants: vec![Object::IntegerObject(1),
                                     Object::IntegerObject(2),
                                     Object::IntegerObject(3),
                                     Object::IntegerObject(4),
                                     Object::IntegerObject(5),
                                     Object::IntegerObject(6)],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpAdd, vec![]).unwrap(),
                make(OpConstant, vec![2]).unwrap(),
                make(OpConstant, vec![3]).unwrap(),
                make(OpSub, vec![]).unwrap(),
                make(OpConstant, vec![4]).unwrap(),
                make(OpConstant, vec![5]).unwrap(),
                make(OpMul, vec![]).unwrap(),
                make(OpArray, vec![3]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]

        }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_hash_expr() {
    let tests = vec![
      CompilerTestCase
      {
          input: "{1: 2, 3: 4, 5: 6}".to_string(),
            expected_constants: vec![
                Object::IntegerObject(1),
                Object::IntegerObject(2),
                Object::IntegerObject(3),
                Object::IntegerObject(4),
                Object::IntegerObject(5),
                Object::IntegerObject(6)
            ],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpConstant, vec![2]).unwrap(),
                make(OpConstant, vec![3]).unwrap(),
                make(OpConstant, vec![4]).unwrap(),
                make(OpConstant, vec![5]).unwrap(),
                make(OpHash, vec![3]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
      }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_index_expressions() {
    let tests = vec![
        CompilerTestCase {
            input: "[1, 2, 3][1 + 1]".to_string(),
            expected_constants: vec![
                IntegerObject(1),
                IntegerObject(2),
                IntegerObject(3),
                IntegerObject(1),
                IntegerObject(1)
            ],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpConstant, vec![2]).unwrap(),
                make(OpArray, vec![3]).unwrap(),
                make(OpConstant, vec![3]).unwrap(),
                make(OpConstant, vec![4]).unwrap(),
                make(OpAdd, vec![]).unwrap(),
                make(OpIndex, vec![]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "{2: 1}[1 + 1]".to_string(),
            expected_constants: vec![
                IntegerObject(2),
                IntegerObject(1),
                IntegerObject(1),
                IntegerObject(1)
            ],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpHash, vec![1]).unwrap(),
                make(OpConstant, vec![2]).unwrap(),
                make(OpConstant, vec![3]).unwrap(),
                make(OpAdd, vec![]).unwrap(),
                make(OpIndex, vec![]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_scopes() {
    let mut compiler = Compiler::new();
    let global_symbol_table = compiler.symbol_table.clone();
    assert_eq!(compiler.scope_index, 0);

    compiler.emit(OpMul, vec![]);
    compiler.enter_scope();

    assert_eq!(compiler.scope_index, 1);

    compiler.emit(OpSub, vec![]);

    assert_eq!(compiler.scopes[compiler.scope_index].instructions.content.len(), 1);

    assert_eq!(compiler.scopes[compiler.scope_index].last_instruction.clone().unwrap().code, OpSub);

    assert_eq!(compiler.symbol_table.outer.clone().unwrap().as_ref().clone(), global_symbol_table);

    compiler.leave_scope();

    assert_eq!(compiler.scope_index, 0);
}
#[test]
fn test_functions() {
    let inputs = vec![
        CompilerTestCase {
            input:
            "fn() { return 5 + 10;}".to_string(),
            expected_constants:
            vec![
                Object::IntegerObject(5),
                Object::IntegerObject(10),
                Object::CompiledFunction(
                    CompiledFunctionStruct {
                        instructions:
                        join_instructions(
                            vec![
                                make(OpConstant, vec![0]).unwrap(),
                                make(OpConstant, vec![1]).unwrap(),
                                make(OpAdd, vec![]).unwrap(),
                                make(code::Opcode::OpReturnValue, vec![]).unwrap()
                            ]
                        ),
                        num_vars: 0,
                        num_args: 0
                    }
                )
            ],
            expected_instructions:
            vec![
                make(code::Opcode::OpClosure, vec![2,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input:
            "fn() { 5 + 10;}".to_string(),
            expected_constants:
            vec![
                Object::IntegerObject(5),
                Object::IntegerObject(10),
                Object::CompiledFunction(
                    CompiledFunctionStruct {
                        instructions:
                        join_instructions(
                            vec![
                                make(OpConstant, vec![0]).unwrap(),
                                make(OpConstant, vec![1]).unwrap(),
                                make(OpAdd, vec![]).unwrap(),
                                make(code::Opcode::OpReturnValue, vec![]).unwrap()
                            ]
                        ), 
                        num_vars: 0,
                        num_args: 0
                    }
                )
            ],
            expected_instructions:
            vec![
                make(code::Opcode::OpClosure, vec![2,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "fn() { 1; 2 }".to_string(),
            expected_constants: vec![
                IntegerObject(1),
                IntegerObject(2),
                Object::CompiledFunction(
                    CompiledFunctionStruct {
                        instructions: join_instructions(
                            vec![
                                make(OpConstant, vec![0]).unwrap(),
                                make(OpPop, vec![]).unwrap(),
                                make(OpConstant, vec![1]).unwrap(),
                                make(code::Opcode::OpReturnValue, vec![]).unwrap()
                            ]
                        ),
                        num_vars: 0,
                        num_args: 0
                    }
                )
            ],
            expected_instructions: vec![
                make(code::Opcode::OpClosure, vec![2,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "fn() { }".to_string(),
            expected_constants: {
                vec![
                    Object::CompiledFunction(
                        CompiledFunctionStruct {
                            instructions: join_instructions(
                                vec![
                                    make(code::Opcode::OpReturn, vec![]).unwrap()
                                ]
                            ),
                            num_vars: 0,
                            num_args: 0
                        }
                    )
                ]
            },
            expected_instructions: {
                vec![
                    make(code::Opcode::OpClosure, vec![0, 0]).unwrap(),
                    make(OpPop, vec![]).unwrap()
                ]
            }
        },
        
    ];

    run_compiler_tests(inputs);
}

#[test]
fn test_function_calls() {
    let tests = vec![
        CompilerTestCase {
            input: "fn() { 24 }();".to_string(),
            expected_constants: vec![
                IntegerObject(24),
                Object::CompiledFunction(
                    CompiledFunctionStruct {
                        instructions: join_instructions(
                            vec![
                                make(OpConstant, vec![0]).unwrap(),
                                make(code::Opcode::OpReturnValue, vec![]).unwrap()
                            ]
                        ),
                        num_vars: 0,
                        num_args: 0
                    }
                )
            ],
            expected_instructions: vec![
                make(code::Opcode::OpClosure, vec![1,0]).unwrap(),
                make(OpCall, vec![0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "let oneArg = fn(a) { a; }; oneArg(24);".to_string(),
            expected_constants: vec![
                Object::CompiledFunction(CompiledFunctionStruct {
                    instructions: join_instructions(vec![
                        make(code::Opcode::OpGetLocal, vec![0]).unwrap(),
                        make(code::Opcode::OpReturnValue, vec![]).unwrap()
                    ]),
                    num_vars: 1,
                    num_args: 1
                })
            ],
            expected_instructions: vec![
                make(code::Opcode::OpClosure, vec![0,0]).unwrap(),
                make(OpSetGlobal, vec![0]).unwrap(),
                make(code::Opcode::OpGetGlobal, vec![0]).unwrap(),
                make(OpConstant, vec![1]).unwrap(),
                make(OpCall, vec![1]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}


#[test]
fn test_let_statement_scopes() {
    let tests = vec![
        CompilerTestCase{
            input: "let num = 55;
                    fn() { num }".to_string(),
            expected_constants: vec![
                IntegerObject(55),
                Object::CompiledFunction(CompiledFunctionStruct{
                    instructions: join_instructions(vec![
                        make(code::Opcode::OpGetGlobal, vec![0]).unwrap(),
                        make(code::Opcode::OpReturnValue, vec![]).unwrap()
                    ]),
                    num_vars: 0,
                    num_args: 0
                })
            ],
            expected_instructions: vec![
                make(OpConstant, vec![0]).unwrap(),
                make(OpSetGlobal, vec![0]).unwrap(),
                make(code::Opcode::OpClosure, vec![1,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "fn() {
                    let num = 55;
                    num 
                    };".to_string(),
            expected_constants: vec![
                IntegerObject(55),
                Object::CompiledFunction(
                    CompiledFunctionStruct {
                        instructions: join_instructions(
                            vec![
                                make(OpConstant, vec![0]).unwrap(),
                                make(code::Opcode::OpSetLocal, vec![0]).unwrap(),
                                make(code::Opcode::OpGetLocal, vec![0]).unwrap(),
                                make(code::Opcode::OpReturnValue, vec![]).unwrap()
                            ]),
                        num_vars: 1,
                        num_args: 0
                    }
                )
            ],
            expected_instructions: vec![
                make(code::Opcode::OpClosure, vec![1,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_builtins() {
    let tests = vec![
        CompilerTestCase {
            input: "len([]);
                    push([], 1);".to_string(),
            expected_constants: vec![
                IntegerObject(1)
            ],
            expected_instructions: vec![
                make(code::Opcode::OpGetBuiltin, vec![0]).unwrap(),
                make(OpArray, vec![0]).unwrap(),
                make(OpCall, vec![1]).unwrap(),
                make(OpPop, vec![]).unwrap(),
                make(code::Opcode::OpGetBuiltin, vec![4]).unwrap(),
                make(OpArray, vec![0]).unwrap(),
                make(OpConstant, vec![0]).unwrap(),
                make(OpCall, vec![2]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        },
        CompilerTestCase {
            input: "fn() { len([]) };".to_string(),
            expected_constants: vec![
                Object::CompiledFunction(CompiledFunctionStruct {
                    instructions: join_instructions(
                        vec![
                            make(code::Opcode::OpGetBuiltin, vec![0]).unwrap(),
                            make(OpArray, vec![0]).unwrap(),
                            make(OpCall, vec![1]).unwrap(),
                            make(code::Opcode::OpReturnValue, vec![]).unwrap()
                        ]
                    ),
                    num_vars: 0,
                    num_args: 0
                })
            ],
            expected_instructions: vec![
                make(code::Opcode::OpClosure, vec![0,0]).unwrap(),
                make(OpPop, vec![]).unwrap()
            ]
        }
    ];

    run_compiler_tests(tests);
}