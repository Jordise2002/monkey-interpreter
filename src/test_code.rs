use crate::code::{make, Opcode};

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