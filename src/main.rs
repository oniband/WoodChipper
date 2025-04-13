use std::env;
use std::fs::File;
use std::io::Read;

mod utils;
use crate::utils::{Instruction, extract_instructions, parse_args};

fn main() -> std::io::Result<()> {
    let file_paths = parse_args(env::args().collect());
    let mut file_handle = File::open(file_paths[0].clone())?;
    let mut data: Vec<u8> = Vec::new();
    if let Ok(value) = file_handle.read_to_end(&mut data) {
        if value > 4096 {
            println!("[WARNING] Binary size exceeds 4KB! Are you sure this is a chip8 ROM?");
        }
    }

    let extracted_instructions: Vec<Instruction> = extract_instructions(data);

    for instruction in extracted_instructions {
        println!("{{");
        println!("  opcode: 0x{:04x}", instruction.opcode);
        println!("  instruction: {:01x}", instruction.instruction);
        println!("  x: 0b{:b}", instruction.x);
        println!("  y: 0b{:b}", instruction.y);
        println!("  n: 0b{:b}", instruction.n);
        println!("  nn: 0b{:b}", instruction.nn);
        println!("  nnn: 0b{:b}", instruction.nnn);
        println!("}}");
    }

    Ok(())
}
