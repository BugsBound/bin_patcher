use anyhow::{Context, Result};
use clap::{App, Arg};
use std::path::PathBuf;

pub struct Config {
    pub file_path: PathBuf,
    pub bytes: Vec<u8>,
    pub address: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        let matches = App::new("BIN Patcher")
            .about("\nThe program allows altering data in a given file, \
                writing bytes to a specified memory address.")
            .author("Author: BugsBound").version("v1.0.1")
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .takes_value(true)
                    .value_name("FILE")
                    .help("Path to BIN File")
                    .required(true),
            )
            .arg(
                Arg::new("bytes")
                    .short('b')
                    .long("bytes")
                    .takes_value(true)
                    .value_name("BYTES")
                    .help("Bytes for write")
                    .required(true),
            )
            .arg(
                Arg::new("address")
                    .short('a')
                    .long("address")
                    .takes_value(true)
                    .value_name("ADDRESS")
                    .help("Address in memory for re-write")
                    .required(true),
            )
            .get_matches();

        let file_path = matches
            .value_of("file")
            .context("Expected file path, found none")?
            .into();

        let raw_bytes = matches
            .value_of("bytes")
            .context("Expected bytes, found none")?;

        let bytes = hex::decode(raw_bytes).context(format!("Invalid bytes: {}", raw_bytes))?;

        let raw_address = matches
            .value_of("address")
            .context("Expected address, found none")?;
        let address = u64::from_str_radix(raw_address.trim_start_matches("0x"), 16)
            .context("Error: The provided address does not match the expected format.")?;

        Ok(Config {
            file_path,
            bytes,
            address,
        })
    }
}
