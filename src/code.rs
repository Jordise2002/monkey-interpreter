use phf::{Map, phf_map};
use byteorder;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

pub type Byte = u8;

pub struct Instruction {
    pub content: Vec<Byte>
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
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


pub fn make(code: Opcode, operands: Vec<usize>) -> Option<Instruction>
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
        Some(Instruction{content:instructions})
    }
    else
    {
        None
    }
}