use std::vec;

use num_traits::FromPrimitive;
use crate::ast::{Expression, Node, Statement};
use crate::code::{Instructions, make, Opcode};
use crate::code::Opcode::{OpAdd, OpArray, OpBang, OpConstant, OpDiv, OpEq, OpFalse, OpGetGlobal, OpGreaterThan, OpHash, OpIndex, OpJump, OpJumpNotTrue, OpMinus, OpMul, OpNotEq, OpNull, OpCall, OpPop, OpSetGlobal, OpSub, OpTrue};
use crate::object::{CompiledFunctionStruct, Object};
use crate::symbol_table::{self, SymbolScope, SymbolTable};
use crate::token::Token;
use crate::builtins::BUILT_INS;
#[derive(PartialEq, Clone, Debug)]
pub struct CompilationScope {
    pub instructions: Instructions,
    pub last_instruction: Option<EmittedInstruction>,
    pub prev_instruction: Option<EmittedInstruction>
}

#[derive(PartialEq, Clone, Debug)]
pub struct EmittedInstruction {
    pub code: Opcode,
    pub index: usize
}

pub struct Compiler {
    pub constants: Vec<Object>,
    pub scopes: Vec<CompilationScope>,
    pub scope_index: usize,
    pub symbol_table: SymbolTable
}


impl Compiler {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        for (builtin, i) in BUILT_INS.iter().zip(0..BUILT_INS.len()) {
            symbol_table.define_builtin(i, builtin.to_string());
        }
        Compiler {
            constants: Vec::new(),
            scopes: vec![CompilationScope{
                instructions: Instructions::new(),
                prev_instruction: None,
                last_instruction: None
            }],
            scope_index: 0,
            symbol_table: symbol_table
        }
    }

    pub fn new_with_state(constants: Vec<Object>, symbol_table: SymbolTable) -> Self {
        Compiler {
            constants,
            scopes: vec![
                CompilationScope{
                    instructions: Instructions::new(),
                    prev_instruction: None,
                    last_instruction: None
                }
            ],
            scope_index: 0,
            symbol_table
        }
    }

    fn get_current_instructions(&self) -> Instructions
    {
        self.scopes[self.scope_index].instructions.clone()
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
        let previous = self.scopes[self.scope_index].last_instruction.clone();
        self.scopes[self.scope_index].last_instruction = Some(EmittedInstruction {
            code,
            index
        });
        self.scopes[self.scope_index].prev_instruction = previous;
    }

    fn is_last_instruction_pop(&self) -> bool {
        self.is_last_instruction(OpPop)
    }
    fn is_last_instruction(&self, code: Opcode) -> bool {
        if let Some(content) = &self.scopes[self.scope_index].last_instruction {
            if code == content.code.clone() {
                return true;
            }
        }
        false
    }

    pub fn emit(& mut self, operation: Opcode,operands: Vec<usize>) -> usize
    {
        let mut instruction = make(operation.clone(), operands).expect("couldn't make instruction");
        let pos = self.add_instructions(& mut instruction);
        self.set_last_instruction(operation, pos);
        pos
    }

    fn add_instructions(&mut self, instruction: & mut Instructions) -> usize
    {
        let pos = self.get_current_instructions().content.len();
        let mut updated = self.get_current_instructions();
        updated.content.append(&mut instruction.content);
        self.scopes[self.scope_index].instructions = updated;
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

                if SymbolScope::Global == symbol.scope
                {
                    self.emit(OpSetGlobal, vec![symbol.index]);

                }
                else if symbol.scope == SymbolScope::Local{
                    self.emit(Opcode::OpSetLocal, vec![symbol.index]);
                }
                
            }
            Statement::ExpressionStatement(expr) =>
                {
                    self.compile_expr(&expr);
                    self.emit(OpPop, vec![]);
                },
            Statement::ReturnStatement(expr) =>
            {
                self.compile_expr(&expr);
                self.emit(Opcode::OpReturnValue, vec![]);
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
            self.scopes[self.scope_index].instructions.content[pos + counter] = new_instruction.content[counter];
            counter += 1;
        }
    }

    fn change_operand(&mut self, pos: usize, operand: usize)
    {
        let op = Opcode::from_u8(self.get_current_instructions().content[pos]).expect("Couldn't read opcode");
        let instruction = make(op, vec![operand]).expect("Couldn't form instruction");

        self.replace_instruction(pos, instruction);
    }
    
    pub fn enter_scope(& mut self) {
        let scope = CompilationScope {
            instructions: Instructions::new(),
            last_instruction: None,
            prev_instruction: None
        };

        self.symbol_table = SymbolTable::new_enclosed(self.symbol_table.clone());
        self.scopes.push(scope);
        self.scope_index = self.scope_index + 1;
    }

    pub fn leave_scope(&mut self) -> Instructions {
        let ins = self.get_current_instructions();
        
        self.symbol_table = self.symbol_table.clone().outer.unwrap().as_ref().clone();
        self.scopes.pop();
        self.scope_index = self.scope_index - 1;

        ins
    }

    fn replace_pop_with_return(&mut self) {
        let last_pos = self.scopes[self.scope_index].last_instruction.clone().unwrap().index;
        self.replace_instruction(last_pos, make(Opcode::OpReturnValue, vec![]).unwrap());

        self.scopes[self.scope_index].last_instruction = Some(EmittedInstruction{
            code: Opcode::OpReturnValue,
            index: last_pos
        })
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
                    self.scopes[self.scope_index].instructions.content.pop();
                }

                let jump_pos = self.emit(OpJump, vec![9999]);
                let after_consequence_pos = self.get_current_instructions().content.len();
                self.change_operand(jump_not_true_pos, after_consequence_pos);

                if let Some(content) = content.alternative.clone()
                {
                    self.compile(Node::StatementBlock(content));

                    if self.is_last_instruction_pop(){
                        self.scopes[self.scope_index].instructions.content.pop();
                    }
                }
                else {
                    self.emit(OpNull, vec![]);
                }

                let after_alternative_pos = self.get_current_instructions().content.len();
                self.change_operand(jump_pos, after_alternative_pos);
            }
            Expression::IntegerExpression(content) =>
                {
                    let constant = Object::IntegerObject(content.clone());
                    let constant_id = self.add_constant(constant);
                    self.emit(OpConstant, vec![constant_id]);
                },
            Expression::CallExpression(content) => 
                {
                    self.compile_expr(&content.function);
                    for arg in &content.args
                    {
                        self.compile_expr(arg);
                    }
                    self.emit(OpCall, vec![content.args.len()]);
                }
            Expression::StringExpression(content) => {
                let constant = Object::StringObject(content.clone());
                let constant_id = self.add_constant(constant);
                self.emit(OpConstant, vec![constant_id]);
            }
            Expression::FnExpression(content) => {
                self.enter_scope();

                for param in &content.params
                {
                    self.symbol_table.define(param.get_id());
                }
                self.compile(Node::StatementBlock(content.body.clone()));
                
                if self.is_last_instruction_pop() {
                    self.replace_pop_with_return();
                }

                if !self.is_last_instruction(Opcode::OpReturnValue) {
                    self.emit(Opcode::OpReturn, vec![]);
                }
                let num_vars = self.symbol_table.num_definitions;
                let instructions = self.leave_scope();
                
                let constant = Object::CompiledFunction(CompiledFunctionStruct{instructions: instructions, num_vars, num_args: content.params.len()});
                let pos = self.add_constant(constant);

                self.emit(Opcode::OpClosure, vec![pos,0]);
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
                if symbol.scope == SymbolScope::Global {
                    self.emit(OpGetGlobal, vec![symbol.index]);
                }
                else if symbol.scope == SymbolScope::Local {
                    self.emit(Opcode::OpGetLocal, vec![symbol.index]);
                }
                else {
                    self.emit(Opcode::OpGetBuiltin, vec![symbol.index]);
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
            instructions: self.get_current_instructions(),
            constants: self.constants.clone()
        }
    }
}

pub struct ByteCode
{
    pub instructions: Instructions,
    pub constants: Vec<Object>
}

