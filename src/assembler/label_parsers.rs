use crate::assembler::Token;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, newline},
    combinator::opt,
    sequence::tuple,
    IResult,
};

pub fn label_declaration(input: &str) -> IResult<&str, Token> {
    let (input, (_, name, _, _)) =
        tuple((multispace0, alphanumeric1, tag(":"), opt(multispace1)))(input)?;

    Ok((
        input,
        Token::LabelDeclaration {
            name: name.to_string(),
        },
    ))
}

pub fn label_usage(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, name, _)) =
        tuple((opt(multispace0), tag("@"), alphanumeric1, opt(newline)))(input)?;

    Ok((
        input,
        Token::LabelUsage {
            name: name.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let input = "test:";
        let expected_token = Token::LabelDeclaration {
            name: "test".to_string(),
        };
        let result = label_declaration(input);
        assert_eq!(result, Ok(("", expected_token)));
    }

    #[test]
    fn test_parse_label_usage() {
        let input = "@test";
        let expected_token = Token::LabelUsage {
            name: "test".to_string(),
        };
        let result = label_usage(input);
        assert_eq!(result, Ok(("", expected_token)));
    }
}
