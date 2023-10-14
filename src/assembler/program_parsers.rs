use nom::{multi::many1, IResult};

use crate::assembler::instruction_parsers::{instruction, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, instructions) = many1(instruction)(input)?;
    Ok((input, Program { instructions }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        assert_eq!(program.instructions.len(), 1);
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode)
    }
}
