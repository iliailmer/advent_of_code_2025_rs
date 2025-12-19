use std::{
    fs::File,
    io::{Error, Read},
};
pub fn read_input_file(file_path: &str) -> Result<String, Error> {
    // read a file
    let mut file: File = File::open(file_path)?;
    let mut puzzle: String = String::new();
    file.read_to_string(&mut puzzle)?;
    Ok(puzzle)
}
