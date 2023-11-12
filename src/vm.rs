use std::io::Cursor;
use std::os::unix::raw::off_t;
use byteorder::{BigEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use crate::code::{Instructions, Opcode};
use crate::compiler::ByteCode;
use crate::object::Object;
use crate::object::Object::{BooleanObject, IntegerObject};

const STACK_SIZE:usize = 2048;
const GLOBAL_SIZE:usize = 65536;

pub struct Vm {
    constants: Vec<Object>,
    instructions: Instructions,

    stack: Vec<Object>,
    sp: usize,

    pub globals: Vec<Option<Object>>
}

impl Vm {
    pub fn new(bytecode: ByteCode) -> Self
    {

        Vm {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::new(),
            sp: 0,
            globals: Vec::new()
        }
    }

    pub fn new_with_state(bytecode: ByteCode, globals: Vec<Option<Object>>) -> Self
    {
        let mut vm = Vm::new(bytecode);
        vm.globals = globals;
        vm
    }
    pub fn get_stack_top(&self) -> Option<Object>
    {
        if self.sp == 0
        {
            None
        }
        else {
            Some(self.stack[self.sp - 1].clone())
        }
    }

    pub fn handle_integer_infix_expression(&mut self) -> (i64, i64)
    {
        let first = self.pop();
        let second = self.pop();
        if let Object::IntegerObject(first) = first
        {
            if let Object::IntegerObject(second) = second {
                return (first, second);
            }
        }
        panic!("Unsupport object type {} {}", first.get_type(), second.get_type())
    }

    pub fn handle_comparison(& mut self, operator: Opcode)
    {
        let second = self.pop();
        let first = self.pop();

        if let IntegerObject(first) = first
        {
            if let IntegerObject(second) = second
            {
                match operator {
                    Opcode::OpEq => {
                        self.push(BooleanObject(first == second))
                    },
                    Opcode::OpNotEq => {
                        self.push(BooleanObject(first != second))
                    },
                    Opcode::OpGreaterThan => {
                        self.push(BooleanObject(first > second))
                    }
                    _ => {
                        panic!("operator not supported {}", operator as u8)
                    }
                }
                return;
            }
        }

        if let BooleanObject(first) = first
        {
            if let BooleanObject(second) = second
            {
                match operator {
                    Opcode::OpEq => {
                        self.push(BooleanObject(first == second))
                    },
                    Opcode::OpNotEq => {
                        self.push(BooleanObject(first != second))
                    },
                    _ => {
                        panic!("operator not supported {}", operator as u8)
                    }
                }
                return;
            }
        }

        panic!("Comparison operands not supported");
    }


    fn handle_prefix(& mut self, operator: Opcode)
    {
        let prev = self.pop();
        match prev
        {
            IntegerObject(content) => {
                match operator {
                    Opcode::OpMinus => {
                        self.push(IntegerObject(-content))
                    },
                    Opcode::OpBang => {
                        if content == 0 {
                            self.push(BooleanObject(false))
                        }
                        else {
                            self.push(BooleanObject(true))
                        }
                    }
                    _ => {
                        panic!("operator not supported {}", operator as u8);
                    }
                }
            },
            BooleanObject(content) => {
                match operator {
                    Opcode::OpBang => {
                        self.push(BooleanObject(!content))
                    }
                    _ => {
                        panic!("operator not supported {}", operator as u8);
                    }
                }
            }
            _ => {
                panic!("operand not supported {}", prev.get_type())
            }
        }
    }

    pub fn push_global(& mut self, element: Object, pos: usize)
    {
        if self.globals.get(pos).is_none()
        {
            let mut i = self.globals.len();
            while i != pos
            {
                self.globals.push(None);
                i += 1;
            }
            self.globals.push(Some(element));
        }
        else {
            self.globals[pos] = Some(element);
        }
    }
    pub fn run(&mut self) {
        let mut cursor = Cursor::new(self.instructions.content.clone());
        while (cursor.position() as usize) < self.instructions.content.len() {
            let opcode = Opcode::from_u8(cursor.read_u8().unwrap()).expect("couldn't convert opcode from u8");
            match opcode
            {
                Opcode::OpConstant => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    self.push(self.constants[index as usize].clone());
                },
                Opcode::OpAdd => {
                    let (first, second) = self.handle_integer_infix_expression();
                    self.push(IntegerObject(first + second));
                },
                Opcode::OpMul => {
                    let (first, second) = self.handle_integer_infix_expression();
                    self.push(IntegerObject(first * second));
                },
                Opcode::OpSub => {
                    let (first, second) = self.handle_integer_infix_expression();
                    self.push(IntegerObject(second - first));
                },
                Opcode::OpDiv => {
                    let (first, second) = self.handle_integer_infix_expression();
                    self.push(IntegerObject(second / first));
                },
                Opcode::OpJumpNotTrue => {
                    let pos = cursor.read_u16::<BigEndian>().unwrap();
                    if !is_true(self.pop())
                    {
                        cursor.set_position(pos as u64);
                    }
                },
                Opcode::OpGetGlobal => {
                    let pos = cursor.read_u16::<BigEndian>().unwrap();
                    if let Some(content) = self.globals.get(pos as usize).clone()
                    {
                        self.push(content.clone().unwrap());
                    }
                },
                Opcode::OpSetGlobal => {
                    let pos = cursor.read_u16::<BigEndian>().unwrap();
                    let element = self.pop();
                    self.push_global(element, pos as usize);
                }
                Opcode::OpJump => {
                    let pos = cursor.read_u16::<BigEndian>().unwrap();
                    cursor.set_position(pos as u64);
                }
                Opcode::OpEq => {
                    self.handle_comparison(opcode);
                },
                Opcode::OpNotEq => {
                    self.handle_comparison(opcode);
                },
                Opcode::OpGreaterThan => {
                    self.handle_comparison(opcode);
                },
                Opcode::OpMinus => {
                    self.handle_prefix(opcode);
                },
                Opcode::OpBang => {
                    self.handle_prefix(opcode);
                },
                Opcode::OpNull =>
                    {
                        self.push(Object::Null);
                    }
                Opcode::OpPop => {
                    self.pop();
                },
                Opcode::OpTrue => {
                    self.push(Object::BooleanObject(true))
                },
                Opcode::OpFalse => {
                    self.push(Object::BooleanObject(false))
                }
                _ => {
                    panic!("Not supported Opcode: {:?}", opcode)
                }
            }

        }
    }

    fn pop(& mut self) -> Object
    {
        let value = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        value
    }

    pub fn last_popped_stack_element(&self) -> Object
    {
        self.stack[self.sp].clone()
    }

    fn push(& mut self, object: Object) {
        if self.sp > STACK_SIZE
        {
            panic!("STACK OVERFLOW");
        }
        if self.stack.get(self.sp).is_none()
        {
            self.stack.push(object);
        }
        else
        {
            self.stack[self.sp] = object;
        }
        self.sp += 1;
    }
}

fn is_true(object: Object) -> bool {
    match object
    {
        IntegerObject(content) =>
            {
                content != 0
            },
        BooleanObject(content) => {
            content
        },
        Object::Null => {
            false
        }
        _ => {
            panic!("type not supported: {}", object.get_type())
        }
    }
}