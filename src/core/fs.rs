use std::fs::{self, File};
use std::io::{self, Write, Read};
use crate::errors::error::CartError;

pub fn write_file(path: &str, content: &str) -> Result<(), CartError> {
    File::create(path)
        .map_err(|e| CartError::IoError(e.to_string()))?
        .write_all(content.as_bytes())
        .map_err(|e| CartError::IoError(e.to_string()))
}

pub fn append_file(path: &str, content: &str) -> Result<(), CartError> {
    use std::fs::OpenOptions;
    OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e| CartError::IoError(e.to_string()))?
        .write_all(content.as_bytes())
        .map_err(|e| CartError::IoError(e.to_string()))
}

pub fn read_file(path: &str) -> Result<String, CartError> {
    let mut file = File::open(path).map_err(|e| CartError::IoError(e.to_string()))?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| CartError::IoError(e.to_string()))?;
    Ok(content)
}

pub fn delete_file(path: &str) -> Result<(), CartError> {
    fs::remove_file(path).map_err(|e| CartError::IoError(e.to_string()))
}
