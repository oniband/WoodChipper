///Returns an array containing:
///[0] -> The path to the binary.
///[1] -> Directory that the dissasembled binary should be placed.
///
/// #Example
///
///  ```no_run
/// use std::env;
/// use std::fs;
///
/// fn main() -> std::io::Result<()> {
///     let paths: [String; 2] = parse_args(env::args().collect());
///
///     let file_handle = fs::File::open(paths[0].clone());
/// }
///
///  ```
pub fn parse_args(args: Vec<String>) -> [String; 2] {
    if args.len() == 1 {
        println!(
            "No Argument(s) given. Usage: woodchipper <Path To File> <Optional Output Directory>"
        );
        panic!("Quitting...");
    } else if args.len() > 3 {
        println!(
            "Too many Argument(s) given. Usage: woodchipper <Path To File> <Optional Output Directory>"
        );
        panic!("Quitting...");
    }
    if args.len() == 3 {
        if args[2].ends_with("/") {
            return [args[1].clone(), args[2].clone()];
        } else {
            return [args[1].clone(), format!("{}/", args[2].clone())];
        }
    }
    [args[1].clone(), "./".to_owned()]
}

///A representation of all possible values within a chip8 opcode
///
///opcode: A two byte instruction
///instruction: The first nibble which defines the type of instruction(draw, jump etc...)
///x: The second nibble which is an address for one of the V registers.
///y: The third nibble which is also an adress for on of the V registers.
///n: The fourth nibble which is a 4 bit number used for data.
///nn: The second byte which is an 8 bit number used for data.
///nnn: The second, third and fourth nibble which is used as a 12bit memory address.
#[derive(Debug)]
pub struct Instruction {
    pub opcode: u16,
    pub instruction: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
    pub first_byte: u8,
    pub second_byte: u8,
}

///Returns a vector of instructions given a vetor of chip8 binary data
pub fn extract_instructions(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut program_counter: usize = 0;
    loop {
        let new_opcode: u16 =
            ((bytes[program_counter] as u16) << 8) | (bytes[program_counter + 1] as u16);

        let new_instruction: Instruction = Instruction {
            opcode: new_opcode,
            instruction: bytes[program_counter] >> 4,
            x: bytes[program_counter] & 0x0F,
            y: bytes[program_counter + 1] >> 4,
            n: bytes[program_counter + 1] & 0x0F,
            nn: bytes[program_counter + 1],
            nnn: new_opcode & 0x0FFF,
            first_byte: bytes[program_counter],
            second_byte: bytes[program_counter + 1],
        };
        instructions.push(new_instruction);

        if (program_counter + 2) > bytes.len() - 1 {
            break;
        } else {
            program_counter += 2;
        }
    }
    instructions
}

pub fn increment_program_counter(program_counter: &mut u16, value: u16) {
    if (*program_counter + value) > 4095 {
        *program_counter = (*program_counter + value) - 4096;
    } else {
        *program_counter += value;
    }
}

pub fn set_program_counter(program_counter: &mut u16, value: u16) {
    if value > 4095 {
        panic!("Invalid program counter value!");
    } else {
        *program_counter = value;
    }
}
