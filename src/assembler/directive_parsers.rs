use crate::assembler::{
    instruction_parsers::AssemblerInstruction, label_parsers::label_declaration,
    operand_parsers::operand, Token,
};
use nom::{
    character::complete::{alpha1, char, multispace0},
    combinator::opt,
    sequence::preceded,
    IResult,
};

fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (input, name) = preceded(char('.'), alpha1)(input)?;
    Ok((
        input,
        Token::Directive {
            name: name.to_string(),
        },
    ))
}

fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, l) = opt(label_declaration)(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, name) = directive_declaration(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, o1) = opt(operand)(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, o2) = opt(operand)(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, o3) = opt(operand)(input)?;

    let directive = AssemblerInstruction {
        opcode: None,
        directive: Some(name),
        label: l,
        operand1: o1,
        operand2: o2,
        operand3: o3,
    };
    Ok((input, directive))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    directive_combined(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_directive() {
        let result = directive_combined("test: .asciiz 'Hello'");
        assert!(result.is_ok());
        let (_, directive) = result.unwrap();

        let correct_instruction = AssemblerInstruction {
            opcode: None,
            label: Some(Token::LabelDeclaration {
                name: "test".to_string(),
            }),
            directive: Some(Token::Directive {
                name: "asciiz".to_string(),
            }),
            operand1: Some(Token::SyString {
                name: "Hello".to_string(),
            }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(directive, correct_instruction);
    }
}
