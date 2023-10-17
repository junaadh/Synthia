use clap::{arg, Parser};
use log::info;
use std::{fs::File, io::Read, path::Path};

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    input_file: Option<String>,
}

fn main() {
    env_logger::init();
    info!("Starting logging!");
    let args = Args::parse();
    let input_file = args.input_file;

    match input_file {
        Some(filename) => {
            let program = read_file(&filename);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            match program {
                Ok(p) => {
                    vm.add_bytes(p);
                    vm.run();
                    std::process::exit(0);
                }
                Err(_) => {}
            }
        }
        None => start_repl(),
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    match File::open(Path::new(&filename)) {
        Ok(mut fh) => {
            let mut contents = String::new();
            match fh.read_to_string(&mut contents) {
                Ok(_) => {
                    return contents;
                }
                Err(e) => {
                    println!("There was an error reading the file: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("File not found: {:?}", e);
            std::process::exit(1);
        }
    }
}
