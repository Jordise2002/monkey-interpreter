use num_traits::FromPrimitive;
use crate::ast::{Expression, Node, Statement};
use crate::code::{Instructions, make, Opcode};
use crate::code::Opcode::{OpAdd, OpArray, OpBang, OpConstant, OpDiv, OpEq, OpFalse, OpGetGlobal, OpGreaterThan, OpHash, OpIndex, OpJump, OpJumpNotTrue, OpMinus, OpMul, OpNotEq, OpNull, OpPop, OpSetGlobal, OpSub, OpTrue};
use crate::object::Object;
use crate::symbol_table::SymbolTable;
use crate::token::Token;


#[derive(PartialEq, Clone, Debug)]
struct EmittedInstruction {
    pub code: Opcode,
    pub index: usize
}

pub struct Compiler {
    instructions: Instructions,
    pub constants: Vec<Object>,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
    pub symbol_table: SymbolTable
}


impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Instructions::new(),
            constants: Vec::new(),
            last_instruction: None,
            previous_instruction: None,
            symbol_table: SymbolTable::new()
        }
    }

    pub fn new_with_state(constants: Vec<Object>, symbol_table: SymbolTable) -> Self {
        Compiler {
            instructions: Instructions::new(),
            constants,
            last_instruction: None,
            previous_instruction: None,
            symbol_table
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
            Node::StatementBlock(stmt_block) => {
                for stmt in stmt_block
                {
                    self.compile(Node::Statement(stmt))
                }
            }
            Node::Statement(stmt) =>
                {
                    self.compile_stmt(stmt)
                },
            Node::Expression(expr) => {
                    self.compile_expr(&expr)
                },

            _ => {
                panic!("Node not supported")
            }
        }
    }

    fn set_last_instruction(& mut self, code: Opcode, index: usize) {
        let previous = self.last_instruction.clone();
        self.last_instruction = Some(EmittedInstruction {
            code,
            index
        });
        self.previous_instruction = previous;
    }

    fn is_last_instruction_pop(&self) -> bool {
        if let Some(content) = &self.last_instruction {
            if let OpPop = content.code.clone() {
                return true;
            }
        }
        false
    }

    fn emit(& mut self, operation: Opcode,operands: Vec<usize>) -> usize
    {
        let mut instruction = make(operation.clone(), operands).expect("couldn't make instruction");
        let pos = self.add_instructions(& mut instruction);
        self.set_last_instruction(operation, pos);
        pos
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
            Statement::LetStatement(id, expr) => {
                self.compile_expr(&expr);
                let symbol = self.symbol_table.define(id.id);
                self.emit(OpSetGlobal, vec![symbol.index]);

            }
            Statement::ExpressionStatement(expr) =>
                {
                    self.compile_expr(&expr);
                    self.emit(OpPop, vec![]);
                }
            _ => {
                panic!("Statement not supported");
            }
        }
    }


    fn replace_instruction(&mut self, pos: usize, new_instruction: Instructions)
    {
        let mut counter = 0;
        while counter < new_instruction.content.len()
        {
            self.instructions.content[pos + counter] = new_instruction.content[counter];
            counter += 1;
        }
    }

    fn change_operand(&mut self, pos: usize, operand: usize)
    {
        let op = Opcode::from_u8(self.instructions.content[pos]).expect("Couldn't read opcode");
        let instruction = make(op, vec![operand]).expect("Couldn't form instruction");

        self.replace_instruction(pos, instruction);
    }

    fn compile_expr(& mut self, expr: &Expression)
    {
        match expr
        {
            Expression::InfixExpression(left, operator, right) =>
                {
                    if operator.clone() == Token::LT {
                        self.compile_expr(right.as_ref());
                        self.compile_expr(left.as_ref());
                    }
                    else {
                        self.compile_expr(left.as_ref());
                        self.compile_expr(right.as_ref());
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
            Expression::IfExpression(content) => {
                self.compile_expr(content.condition.as_ref());
                let jump_not_true_pos = self.emit(OpJumpNotTrue, vec![9999]);
                self.compile(Node::StatementBlock(content.consequence.clone()));
                if self.is_last_instruction_pop() {
                    self.instructions.content.pop();
                }

                let jump_pos = self.emit(OpJump, vec![9999]);
                let after_consequence_pos = self.instructions.content.len();
                self.change_operand(jump_not_true_pos, after_consequence_pos);

                if let Some(content) = content.alternative.clone()
                {
                    self.compile(Node::StatementBlock(content));

                    if self.is_last_instruction_pop(){
                        self.instructions.content.pop();
                    }
                }
                else {
                    self.emit(OpNull, vec![]);
                }

                let after_alternative_pos = self.instructions.content.len();
                self.change_operand(jump_pos, after_alternative_pos);
            }
            Expression::IntegerExpression(content) =>
                {
                    let constant = Object::IntegerObject(content.clone());
                    let constant_id = self.add_constant(constant);
                    self.emit(OpConstant, vec![constant_id]);
                },
            Expression::StringExpression(content) => {
                let constant = Object::StringObject(content.clone());
                let constant_id = self.add_constant(constant);
                self.emit(OpConstant, vec![constant_id]);
            }
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
                    self.compile_expr(inner_expr.as_ref());
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
            },
            Expression::ArrayLiteral(content) => {
                for expr in &content.elements
                {
                    self.compile_expr(expr)
                }

                self.emit(OpArray, vec![content.elements.len()]);
            },
            Expression::HashExpression(content) => {
                for (key, value) in &content.pairs{
                    self.compile_expr(key);
                    self.compile_expr(value);
                }

                self.emit(OpHash, vec![content.pairs.len()]);
            },
            Expression::IndexExpression(content) => {
                self.compile_expr(content.left.as_ref());
                self.compile_expr(content.index.as_ref());

                self.emit(OpIndex, vec![]);
            }
            Expression::IdentifierExpression(id) => {
                let symbol = self.symbol_table.resolve(id.id.clone()).expect(format!("undefined variable {}", id.id).as_str());
                self.emit(OpGetGlobal, vec![symbol.index]);
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

