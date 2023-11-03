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
        while cursor.position() < (self.content.len() - 1) as u64
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
    OpConstant
}

pub struct Definition {
    pub name: String,
    pub operand_withs: Vec<u32>
}

pub fn look_up(code: &Opcode) -> Option<Definition>
{
    match code {
       _OpConstant => {
            Some(Definition{ name: "OpConstant".to_string(), operand_withs: vec![2]})
        },
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