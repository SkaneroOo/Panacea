use crate::assembler::{
    Token, 
    opcode_parser::*, 
    operand_parser::operand
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

        while result.len() < 4 {
            result.push(0);
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
        op: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: op,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Opcode;

    #[test]
    fn test_parse_single_instruction() {
        let result = instruction(CompleteStr("hlt"));
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

    #[test]
    fn test_parse_instruction_with_parameters() {
        let result = instruction(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}