use crate::code::{make, Opcode};
use crate::code::Opcode::{OpAdd, OpConstant};
use crate::test_compiler::concat_instructions;


struct MakeTest {
    input_op_code: Opcode,
    input_operands: Vec<usize>,
    expected: Vec<u8>
}
#[test]
fn test_make() {
    let tests = vec![
        MakeTest {
            input_op_code: OpConstant,
            input_operands: vec![65534],
            expected: vec![OpConstant as u8, 255, 254]
        },
        MakeTest {
            input_op_code: OpAdd,
            input_operands: vec![],
            expected: vec![OpAdd as u8]
        }
    ];

    for test in tests {
        let result = make(test.input_op_code, test.input_operands).expect("Couldn't parse code");
        for (i, o) in result.content.clone().into_iter().zip(test.expected)
        {
            assert_eq!(i, o,);
        }
    }
}

#[test]
fn test_print_code() {
    let input = vec![make(OpConstant, vec![87]).unwrap(),
        make(OpConstant, vec![3]).unwrap(),
        make(OpAdd, vec![]).unwrap()];
    let input = concat_instructions(input);
    let expected = "0000 OpConstant 87\n0003 OpConstant 3\n0006 OpAdd\n";
    print!("{}",input.to_string());
    assert_eq!(input.to_string(), expected);

}

#[test]
fn test_op_add()
{

}
