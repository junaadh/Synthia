use crate::{assembler::Token, instruction::Opcode};
use nom::{character::complete::alpha1, combinator::map, IResult};

pub fn opcode(input: &str) -> IResult<&str, Token> {
    map(alpha1, |opcode| Token::Op {
        code: Opcode::from(opcode),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode("load");
        assert_eq!(result, Ok(("", Token::Op { code: Opcode::LOAD })));
        let result = opcode("aold");
        assert_eq!(result, Ok(("", Token::Op { code: Opcode::IGL })));
    }
}
