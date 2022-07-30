use nom::types::CompleteStr;
use nom::*;
use crate::assembler::Token;
use crate::vm::Opcode;

named!(pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}