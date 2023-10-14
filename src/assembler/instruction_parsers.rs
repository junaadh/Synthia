use nom::{
    branch::alt,
    character::complete::{multispace0, newline},
    combinator::opt,
    IResult,
};

use crate::assembler::{
    opcode_parsers::*, operand_parsers::integer_operand, register_parsers::register, Token,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    // pub label: Option<Token>,
    // pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode not found in opcode field");
                std::process::exit(1);
            }
        }
    }
}

fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, o) = opcode(input)?;
    let (input, _) = multispace0(input)?;
    let (input, r) = register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, i) = integer_operand(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        },
    ))
}

fn instruction_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, o) = opcode(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: None,
            operand2: None,
            operand3: None,
        },
    ))
}

fn instruction_three(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, o) = opcode(input)?;
    let (input, _) = multispace0(input)?;
    let (input, r1) = register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, r2) = register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, r3) = register(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r1),
            operand2: Some(r2),
            operand3: Some(r3),
        },
    ))
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((instruction_three, instruction_one, instruction_two))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two("hlt\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        )
    }

    #[test]
    fn test_parse_instructions_form_three() {
        let result = instruction_three("add $0 $1 $2");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 })
                }
            ))
        )
    }
}
