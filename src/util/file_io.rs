use anyhow::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

pub fn write_to_path(path: &str, bytes: &[u8]) -> Result<()> {
    let path = Path::new(path);
    let mut file = File::create(path).context("couldn't create file")?;
    file.write_all(bytes).context("couldn't write to file")?;
    Ok(())
}

pub fn read_file_to_string(path: &str) -> Result<String> {
    let file = File::open(path).with_context(|| format!("couldn't read file path {}", path))?;

    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str);
    Ok(str)
}
