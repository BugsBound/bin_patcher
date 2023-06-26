use goblin::elf::Elf;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

pub fn vaddr_to_file_offset(elf: &Elf, vaddr: u64) -> Option<u64> {
    for header in &elf.program_headers {
        if header.p_type == goblin::elf::program_header::PT_LOAD
            && vaddr >= header.p_vaddr
            && vaddr < (header.p_vaddr + header.p_memsz)
        {
            return Some(header.p_offset + (vaddr - header.p_vaddr));
        }
    }
    None
}

pub fn run(file_path: &PathBuf, offset: u64, bytes: &[u8]) -> anyhow::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    file.seek(SeekFrom::Start(offset))?;
    file.write_all(bytes)?;
    file.flush()?;
    file.sync_data()?;
    file.sync_all()?;

    Ok(())
}
