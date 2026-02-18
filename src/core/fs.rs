use std::fs::{self, File};
use std::io::{Write, Read};
use crate::errors::error::CartError;

pub fn write_file(path: &str, content: &str) -> Result<(), CartError> {
    File::create(path)
        .map_err(|e: std::io::Error| CartError::IoError(e.to_string()))?
        .write_all(content.as_bytes())
        .map_err(|e: std::io::Error| CartError::IoError(e.to_string()))
}

pub fn append_file(path: &str, content: &str) -> Result<(), CartError> {
    use std::fs::OpenOptions;
    OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e: std::io::Error| CartError::IoError(e.to_string()))?
        .write_all(content.as_bytes())
        .map_err(|e: std::io::Error| CartError::IoError(e.to_string()))
}

pub fn read_file(path: &str) -> Result<String, CartError> {
    let mut file: File = File::open(path).map_err(|e: std::io::Error| CartError::IoError(e.to_string()))?;
    let mut content: String = String::new();
    file.read_to_string(&mut content).map_err(|e: std::io::Error| CartError::IoError(e.to_string()))?;
    Ok(content)
}

pub fn delete_file(path: &str) -> Result<(), CartError> {
    fs::remove_file(path).map_err(|e: std::io::Error| CartError::IoError(e.to_string()))
}

pub fn copy_file(path: &str, dest: &str) -> Result<(), CartError> { return; }

pub fn enfy(path: &str, to: &str) -> Result<(), CartError> { return; }
