use nom::{
    branch::alt,
    character::complete::{multispace0, newline},
    combinator::opt,
    IResult,
};

use crate::assembler::{
    directive_parsers::directive, label_parsers::label_declaration, opcode_parsers::*,
    operand_parsers::operand, SymbolTable, Token,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(ref token) => match token {
                Token::Op { code } => match code {
                    _ => {
                        results.push(*code as u8);
                    }
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                    std::process::exit(1);
                }
            },
            None => {}
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols);
            }
        }
        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn get_label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(d) => match d {
                Token::Directive { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn has_operands(&self) -> bool {
        self.operand1.is_some() || self.operand2.is_some() || self.operand3.is_some()
    }

    pub fn get_string_constant(&self) -> Option<String> {
        match &self.operand1 {
            Some(d) => match d {
                Token::SyString { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
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
            Token::LabelUsage { name } => match symbols.symbol_value(name) {
                Some(value) => {
                    let byte1 = value;
                    let byte2 = value >> 8;
                    results.push(byte2 as u8);
                    results.push(byte1 as u8);
                }
                None => {}
            },
            _ => {
                println!("Opcode not found in opcode field");
                std::process::exit(1);
            }
        };
    }
}

fn instruction_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, l) = opt(label_declaration)(input)?;
    let (input, _) = opt(multispace0)(input)?;

    let (input, o) = opcode(input)?;
    let (input, _) = multispace0(input)?;

    let (input, o1) = opt(operand)(input)?;
    let (input, _) = opt(multispace0)(input)?;

    let (input, o2) = opt(operand)(input)?;
    let (input, _) = opt(multispace0)(input)?;

    let (input, o3) = opt(operand)(input)?;
    let (input, _) = opt(multispace0)(input)?;

    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(o),
            label: l,
            directive: None,
            operand1: o1,
            operand2: o2,
            operand3: o3,
        },
    ))
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((instruction_combined, directive))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction("load $0 @test1\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::LabelUsage {
                        name: "test1".to_string()
                    }),
                    operand3: None
                }
            ))
        )
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction("hlt");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    label: None,
                    directive: None,
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        )
    }

    #[test]
    fn test_parse_instructions_form_three() {
        let result = instruction("add $0 $1 $2");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 })
                }
            ))
        )
    }

    #[test]
    fn test_directive() {
        let result = instruction(".data\nhello: .asciiz 'Hello'\n.code\nhlt");
        assert!(result.is_ok());
    }
}
