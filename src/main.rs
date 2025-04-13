use std::env;
use std::fs::File;
use std::io::{Read, Write};

mod utils;
use crate::utils::{Instruction, extract_instructions, parse_args};

fn main() -> std::io::Result<()> {
    //Check the rom exists
    let file_paths: [String; 2] = parse_args(env::args().collect());
    let mut rom_file_handle: File = File::open(&file_paths[0])?;
    //Load the contents of the file into a vector
    let mut rom_binary_data: Vec<u8> = Vec::new();
    if let Ok(value) = rom_file_handle.read_to_end(&mut rom_binary_data) {
        if value > 4096 {
            println!("[WARNING] Binary size exceeds 4KB! Are you sure this is a chip8 ROM?");
        }
    }

    //Extract opcodes and instructions from the binary and write translated code to file
    let extracted_instructions: Vec<Instruction> = extract_instructions(rom_binary_data);
    let mut assembly_file_handle: File =
        File::create(format!("{}woodchipper_output", &file_paths[1]))?;

    for instruction in extracted_instructions {
        match instruction.instruction {
            0x0 => match instruction.y {
                0xE => {
                    assembly_file_handle.write("CLS\n".as_bytes());
                }
                _ => {
                    assembly_file_handle
                        .write(format!("DATA: {}\n", instruction.first_byte).as_bytes());
                    assembly_file_handle
                        .write(format!("DATA: {}\n", instruction.second_byte).as_bytes());
                }
            },
            0x1 => {
                assembly_file_handle.write(format!("JUMP 0x{:03X}\n", instruction.nnn).as_bytes());
            }
            0x6 => {
                assembly_file_handle
                    .write(format!("MOVE V{}, {}\n", instruction.x, instruction.nn).as_bytes());
            }
            0x7 => {
                assembly_file_handle
                    .write(format!("ADD V{}, {}\n", instruction.x, instruction.nn).as_bytes());
            }
            0xA => {
                assembly_file_handle.write(format!("MOVE I, {}\n", instruction.nnn).as_bytes());
            }
            0xD => {
                assembly_file_handle.write(
                    format!(
                        "DRAW, V{} V{} FROM I FOR {} ROWS\n",
                        instruction.x, instruction.y, instruction.n
                    )
                    .as_bytes(),
                );
            }
            _ => {
                assembly_file_handle
                    .write(format!("DATA: {}\n", instruction.first_byte).as_bytes());
                assembly_file_handle
                    .write(format!("DATA: {}\n", instruction.second_byte).as_bytes());
            }
        }
    }

    Ok(())
}
