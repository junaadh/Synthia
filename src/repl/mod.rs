use std::{
    fs::File,
    io::{self, Read, Write},
    num::ParseIntError,
    path::Path,
    u8,
};

use crate::{
    assembler::{program_parsers::program, Assembler},
    vm::VM,
};

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
            asm: Assembler::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Synthia! Language vm in rust!");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Exiting...    [Exited]");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing");
                }
                ".clear_program" => {
                    println!("Removing all bytes from program VM vector... ");
                    self.vm.program.truncate(0);
                    println!("Job Done!");
                }
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to readline from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("There was an error reading from the file");
                    match self.asm.assemble(&contents) {
                        Some(mut assembled_program) => {
                            println!("Sending assembled program to VM");
                            self.vm.program.append(&mut assembled_program);
                            print!("{:#?}", self.vm.program);
                            self.vm.run();
                        }
                        None => {
                            println!("Unable to parse input");
                            continue;
                        }
                    }
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_remainder, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };

                    self.vm
                        .program
                        .append(&mut program.to_bytes(&self.asm.symbols));
                    self.vm.run_once();
                }
            }
        }
    }

    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
