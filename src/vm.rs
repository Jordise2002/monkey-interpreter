use std::io::Cursor;
use std::os::unix::raw::off_t;
use byteorder::{BigEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use crate::code::{Instructions, Opcode};
use crate::compiler::ByteCode;
use crate::object::Object;

const STACK_SIZE:usize = 2048;

pub struct Vm {
    constants: Vec<Object>,
    instructions: Instructions,

    stack: Vec<Object>,
    sp: usize
}

impl Vm {
    pub fn new(bytecode: ByteCode) -> Self
    {
        Vm {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::new(),
            sp: 0

        }
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

    pub fn run(&mut self) {
        let mut cursor = Cursor::new(self.instructions.content.clone());
        while (cursor.position() as usize) < self.instructions.content.len() -1 {
            let opcode = Opcode::from_u8(cursor.read_u8().unwrap()).expect("couldn't convert opcode from u8");
            match opcode
            {
                Opcode::OpConstant => {
                    let index = cursor.read_u16::<BigEndian>().unwrap();
                    self.push(self.constants[index as usize].clone());
                }
            }

        }
    }

    fn push(& mut self, object: Object) {
        if self.sp > STACK_SIZE
        {
            panic!("STACK OVERFLOW");
        }
        self.stack.push(object);
        self.sp += 1;
    }
}