use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
enum FileError {
    #[error("file error: {0}")]
    IOError(#[from] io::Error),
}

fn read_file_contents(file_path: &Path) -> Result<String, FileError> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    
    buf_reader.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn main() -> Result<(), FileError> {
    let file_path = Path::new("resources/input1.txt");
    let contents = read_file_contents(file_path)?;

    let anagrams: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    
    println!("{:?}", anagrams);

    Ok(())
}
