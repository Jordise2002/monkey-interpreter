use crate::code::{make, Opcode};
use crate::code::Opcode::OpConstant;
use crate::test_compiler::concat_instructions;

#[test]
fn test_make() {
    let input = (
        Opcode::OpConstant,
        vec![65534]);
    let expected = vec![
        Opcode::OpConstant as u8,
        255,
        254
    ];
    let instruction = make(input.0, input.1);
    if let Some(instruction) = instruction
    {
        assert_eq!(instruction.content.len(), 3);
        for (i,e) in instruction.content.into_iter().zip(expected)
        {
            assert_eq!(i,e);
        }
    }
}

#[test]
fn test_print_code() {
    let input = vec![make(OpConstant, vec![87]).unwrap(),
        make(OpConstant, vec![3]).unwrap()];
    let input = concat_instructions(input);
    let expected = "0000 OpConstant 87\n0003 OpConstant 3\n";
    //print!("{}",input.to_string());
    assert_eq!(input.to_string(), expected);

}
