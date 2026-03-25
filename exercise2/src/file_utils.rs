use anyhow::Result;
use std::fs;
use std::path::Path;

/// Read contents of file.
///
/// * `input_file`: filename
/// * returns: contents of file as byte Array
pub fn read_from_file<P: AsRef<Path>>(input_file: P) -> Result<Vec<u8>> {
    let data = fs::read(input_file)?;
    Ok(data)
}

/// Write data into a file.
///
/// * `dest_file`: filename
/// * `data`: byte Array to save
pub fn write_to_file<P: AsRef<Path>>(dest_file: P, data: &[u8]) -> Result<()> {
    fs::write(dest_file, data)?;
    Ok(())
}
