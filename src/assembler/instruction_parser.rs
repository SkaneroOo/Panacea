use crate::assembler::{
    Token, 
    opcode_parser::*, 
    operand_parser::integer_operand,
    register_parser::register
};

use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    result.push(code as u8);
                }
            },
            _ => {
                println!("No opcode found for instruction");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut result)
            }
        }

        return result
    }

    fn extract_operand(t: &Token, result: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                result.push(*reg_num);
            },
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                result.push(byte2 as u8);
                result.push(byte1 as u8);
            }
            _ => {
                println!("Opcode in operand field");
                std::process::exit(1);
            }
        };
    }
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}