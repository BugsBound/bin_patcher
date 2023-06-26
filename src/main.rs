mod objects;
mod patch;

use anyhow::Result;
use goblin::elf::Elf;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let config = objects::Config::new()?;

    let mut fd = File::open(&config.file_path)?;

    let mut buffer = Vec::new();
    fd.read_to_end(&mut buffer)?;

    let elf = Elf::parse(&buffer)?;

    match patch::vaddr_to_file_offset(&elf, config.address) {
        Some(file_offset) => {
            patch::run(&config.file_path, file_offset, &config.bytes)?;
            println!("Successfully patched file.");
        }
        None => {
            println!(
                "No corresponding file offset for vaddr {:x}",
                config.address
            );
        }
    }

    Ok(())
}
