use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    IResult,
};
use std::str::FromStr;

use crate::assembler::{register_parsers::register, Token};

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('#')(input)?;
    let (input, value) = map_res(digit1, i32::from_str)(input)?;

    Ok((input, Token::IntegerOperand { value }))
}

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((integer_operand, register))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integar_operand() {
        let result = integer_operand("#10");
        assert_eq!(result, Ok(("", Token::IntegerOperand { value: 10 })));
        let result = integer_operand("10");
        assert_ne!(result, Ok(("", Token::IntegerOperand { value: 10 })));
        let result = integer_operand("#a");
        assert!(result.is_err());
    }
}
