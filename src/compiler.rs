use crate::ast::{Expression, Node, Statement};
use crate::code::{Instructions, make, Opcode};
use crate::code::Opcode::{OpAdd, OpBang, OpConstant, OpDiv, OpEq, OpFalse, OpGreaterThan, OpMinus, OpMul, OpNotEq, OpPop, OpSub, OpTrue};
use crate::object::Object;
use crate::token::Token;

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>
}


impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Instructions::new(),
            constants: Vec::new()
        }
    }

    pub fn compile(& mut self, node: Node)
    {
        match node {
            Node::Program(prog) =>
                {
                    for stmt in prog.statements
                    {
                        self.compile(Node::Statement(stmt))
                    }
                },
            Node::Statement(stmt) =>
                {
                    self.compile_stmt(stmt)
                },
            Node::Expression(expr) => {
                    self.compile_expr(expr)
                },

            _ => {
                panic!("Node not supported")
            }
        }
    }
    fn emit(& mut self, operation: Opcode,operands: Vec<usize>) -> usize
    {
        let mut instruction = make(operation, operands).expect("couldn't make instruction");
        self.add_instructions(& mut instruction)
    }

    fn add_instructions(&mut self, instruction: & mut Instructions) -> usize
    {
        let pos = self.instructions.content.len();
        self.instructions.content.append(& mut instruction.content);
        pos
    }

    fn add_constant(&mut self, constant: Object) -> usize
    {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    fn compile_stmt(& mut self, stmt: Statement)
    {
        match stmt
        {
            Statement::ExpressionStatement(expr) =>
                {
                    self.compile_expr(expr);
                    self.emit(OpPop, vec![]);
                }
            _ => {
                panic!("Statement not supported");
            }
        }
    }

    fn compile_expr(& mut self, expr: Expression)
    {
        match expr
        {
            Expression::InfixExpression(left, operator, right) =>
                {
                    if operator == Token::LT {
                        self.compile_expr(right.as_ref().clone());
                        self.compile_expr(left.as_ref().clone());
                    }
                    else {
                        self.compile_expr(left.as_ref().clone());
                        self.compile_expr(right.as_ref().clone());
                    }

                    match operator
                    {
                        Token::PLUS =>
                            {
                                self.emit(OpAdd, vec![]);
                            }
                        Token::MINUS => {
                                self.emit(OpSub, vec![]);
                            },
                        Token::SLASH => {
                            self.emit(OpDiv, vec![]);
                        },
                        Token::ASTERISK => {
                            self.emit(OpMul, vec![]);
                        },
                        Token::EQ => {
                            self.emit(OpEq, vec![]);
                        },
                        Token::NotEq => {
                            self.emit(OpNotEq, vec![]);
                        },
                        Token::GT => {
                            self.emit(OpGreaterThan, vec![]);
                        },
                        Token::LT => {
                            self.emit(OpGreaterThan, vec![]);
                        }
                        _ => {
                            panic!("operator not suported {}", operator.inspect())
                        }
                    }
                },
            Expression::IntegerExpression(content) =>
                {
                    let constant = Object::IntegerObject(content);
                    let constant_id = self.add_constant(constant);
                    self.emit(OpConstant, vec![constant_id]);
                },
            Expression::BoolExpression(content) =>
                {
                    match content
                    {
                        true =>
                            {
                                self.emit(OpTrue, vec![]);
                            },
                        false => {
                            self.emit(OpFalse, vec![]);
                        }
                    }
                },
            Expression::PrefixExpression(operator, inner_expr) =>
                {
                    self.compile_expr(inner_expr.as_ref().clone());
                    match operator {
                        Token::BANG => {
                            self.emit(OpBang, vec![]);
                        },
                        Token::MINUS => {
                            self.emit(OpMinus, vec![]);
                        }
                        _ => {
                            panic!("Operator {} not supported", operator.inspect());
                        }
                    }
            }
            _=> {
                panic!("Expression not supported");
            }
        }
    }

    pub fn get_bytecode(&self) -> ByteCode
    {
        ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone()
        }
    }
}

pub struct ByteCode
{
    pub instructions: Instructions,
    pub constants: Vec<Object>
}

