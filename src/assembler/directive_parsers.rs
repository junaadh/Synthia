use crate::assembler::{
    instruction_parsers::AssemblerInstruction, operand_parsers::operand, Token,
};
use nom::{
    character::complete::{alpha1, char, multispace0, multispace1},
    combinator::opt,
    sequence::{pair, preceded},
    IResult,
};

fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (input, (_, name)) = pair(preceded(char('.'), alpha1), multispace1)(input)?;
    Ok((
        input,
        Token::Directive {
            name: name.to_string(),
        },
    ))
}

fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
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
        label: None,
        operand1: o1,
        operand2: o2,
        operand3: o3,
    };
    Ok((input, directive))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    directive_combined(input)
}
