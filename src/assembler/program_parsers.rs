use nom::{multi::many1, IResult};

use crate::assembler::{
    instruction_parsers::{instruction, AssemblerInstruction},
    SymbolTable,
};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes(symbols));
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
        let symbols = SymbolTable::new();
        let bytecode = program.to_bytes(&symbols);
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode)
    }

    #[test]
    fn test_complete_program() {
        let test_program = ".data\nhello: .ascizz 'Hello Everyone!'\n.code\nhlt";
        let result = program(test_program);
        assert!(result.is_ok());
    }
}
