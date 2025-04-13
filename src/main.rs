use std::env;
use std::fs::File;
use std::io::Read;

mod utils;
use crate::utils::utils::parse_args;

fn main() -> std::io::Result<()> {
    let file_paths = parse_args(env::args().collect());
    let mut file_handle = File::open(file_paths[0].clone())?;
    let mut data: Vec<u8> = Vec::new();
    match file_handle.read_to_end(&mut data) {
        Ok(value) => {
            if value > 4096 {
                println!("[WARNING] Binary size exceeds 4KB! Are you sure this is a chip8 ROM?");
            }
        }
        Err(..) => (),
    }

    for line in data {
        println!("{line:x}");
    }

    Ok(())
}
