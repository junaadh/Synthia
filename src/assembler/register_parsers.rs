use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    IResult,
};
use std::str::FromStr;

use crate::assembler::Token;

pub fn register(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('$')(input)?;
    let (input, reg_num) = map_res(digit1, u8::from_str)(input)?;

    Ok((input, Token::Register { reg_num }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result, Ok(("", Token::Register { reg_num: 0 })));
        let result = register("0");
        assert!(result.is_err());
        let result = register("$a");
        assert!(result.is_err());
    }
}
