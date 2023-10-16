use nom::{
    branch::alt,
    character::complete::{alphanumeric0, char, digit1},
    combinator::map_res,
    multi::many0,
    IResult,
};
use std::str::FromStr;

use crate::assembler::{label_parsers::label_usage, register_parsers::register, Token};

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('#')(input)?;
    let (input, value) = map_res(digit1, i32::from_str)(input)?;

    Ok((input, Token::IntegerOperand { value }))
}

fn systring(input: &str) -> IResult<&str, Token> {
    let (input, quote) = alt((char('\''), char('"')))(input)?;
    let (input, content) = (alphanumeric0)(input)?;
    let (input, _) = char(quote)(input)?;

    Ok((
        input,
        Token::SyString {
            name: content.to_string(),
        },
    ))
}

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((integer_operand, register, label_usage, systring))(input)
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

    #[test]
    fn test_parse_string_single_quotes() {
        let result = systring("'hello'");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_string_double_quotes() {
        let result = systring("\"hello\"");
        assert!(result.is_ok());
    }
}
