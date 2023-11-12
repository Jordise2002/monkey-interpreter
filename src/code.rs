use std::fmt::{Display,Formatter};
use std::io::Cursor;
use byteorder;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type Byte = u8;

#[derive(Clone, Debug, PartialEq)]
pub struct Instructions {
    pub content: Vec<Byte>
}

impl Instructions {
    pub fn new() -> Self {
        Instructions {
            content: Vec::new()
        }
    }
}

impl Display for Instructions
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut len = 0;
        let mut output = String::new();
        let mut cursor = Cursor::new(self.content.clone());
        while cursor.position() < (self.content.len()) as u64
        {
            let code_value = cursor.read_u8().unwrap();
            let code = Opcode::from_u8(code_value);
            output += format!("{:0>4} ", len).as_str();
            len += 1;
            if let None = code {
                output += format!("Opcode not supported {}", code_value).as_str();
                continue;
            }
            let code = code.unwrap();
            let def = look_up(&code);
            if let Some(def) = def
            {
                output += def.name.as_str();
                output += " ";
                for i in def.operand_withs
                {
                    match i
                    {
                        1 => {
                            output += format!("{} ", cursor.read_u8().unwrap()).as_str();
                            len += 1;
                        },
                        2 => {
                            output += format!("{} ", cursor.read_u16::<BigEndian>().unwrap()).as_str();
                            len += 2;
                        },
                        _ => {
                            output += format!("Error operand with not supported {}", i).as_str();
                        }
                    }
                }
                output.pop();
                output += "\n";
            }
            else {
                output += format!("Opcode not supported {}", code as u8).as_str();
            }
        }
        write!(f, "{}", output)
    }
}
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum Opcode {
    OpConstant,
    OpAdd,
    OpPop,
    OpSub,
    OpDiv,
    OpMul,
    OpTrue,
    OpFalse,
    OpEq,
    OpNotEq,
    OpGreaterThan,
    OpMinus,
    OpBang,
    OpJumpNotTrue,
    OpJump,
    OpNull,
    OpSetGlobal,
    OpGetGlobal,
    OpArray,
    OpHash,
    OpIndex
}

pub struct Definition {
    pub name: String,
    pub operand_withs: Vec<u32>
}

pub fn look_up(code: &Opcode) -> Option<Definition>
{
    match code {
       Opcode::OpConstant => {
            Some(Definition{ name: "OpConstant".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpAdd => {
           Some(Definition{name: "OpAdd".to_string(), operand_withs: vec![]})
        },
        Opcode::OpPop => {
            Some(Definition{name: "OpPop".to_string(), operand_withs: vec![]})
        },
        Opcode::OpDiv => {
            Some(Definition{name: "OpDiv".to_string(), operand_withs: vec![]})
        },
        Opcode::OpSub => {
            Some(Definition{name: "OpSub".to_string(), operand_withs: vec![]})
        },
        Opcode::OpMul => {
            Some(Definition{name: "OpMul".to_string(), operand_withs: vec![]})
        },
        Opcode::OpFalse => {
            Some(Definition{name: "OpFalse".to_string(), operand_withs: vec![]})
        },
        Opcode::OpTrue => {
            Some(Definition{name: "OpTrue".to_string(), operand_withs: vec![]})
        },
        Opcode::OpEq => {
            Some(Definition{name: "OpEq".to_string(), operand_withs: vec![]})
        },
        Opcode::OpNotEq => {
            Some(Definition{name: "OpNotEq".to_string(), operand_withs: vec![]})
        },
        Opcode::OpGreaterThan => {
            Some(Definition{name:"OpGreaterThan".to_string(), operand_withs: vec![]})
        },
        Opcode::OpBang => {
            Some(Definition{name:"OpBang".to_string(), operand_withs: vec![]})
        },
        Opcode::OpMinus => {
            Some(Definition{name:"OpMinus".to_string(), operand_withs: vec![]})
        },
        Opcode::OpJump => {
            Some(Definition{name:"OpJump".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpJumpNotTrue => {
            Some(Definition{name:"OpJumpNotTrue".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpNull => {
            Some(Definition{name:"OpNull".to_string(), operand_withs: vec![]})
        },
        Opcode::OpGetGlobal => {
            Some(Definition{name:"OpGetGlobal".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpSetGlobal => {
            Some(Definition{name:"OpSetGlobal".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpArray => {
            Some(Definition{name:"OpArray".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpHash => {
            Some(Definition{name:"OpHash".to_string(), operand_withs: vec![2]})
        },
        Opcode::OpIndex => {
            Some(Definition{name:"OpIndex".to_string(), operand_withs: vec![]})
        }
        _ => {
            None
        }
    }
}


pub fn make(code: Opcode, operands: Vec<usize>) -> Option<Instructions>
{
    let mut instructions = Vec::new();
    instructions.push(code.clone() as u8);
    if let Some(content) = look_up(&code)
    {
        for (i,o) in content.operand_withs.into_iter().zip(operands)
        {
            match i {
                2 => {
                    instructions.write_u16::<BigEndian>(o as u16).unwrap();
                },
                1 => {
                    instructions.write_u8(o as u8).unwrap();
                }
                _ => {
                    panic!("unsupported operand width: {}", i);
                }
            }

        }
        Some(Instructions {content:instructions})
    }
    else
    {
        None
    }
}