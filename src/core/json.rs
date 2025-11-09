use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;

pub trait JSON {
    fn create_file(filename: &str, path: &str) -> std::io::Result<()>;
    fn write_to_json<T: Serialize>(path: &str, data: &T) -> std::io::Result<()>;
    fn read_from_json<T: for<'de> Deserialize<'de>>(path: &str) -> std::io::Result<T>;
}

pub struct JsonHandler;

impl JSON for JsonHandler {
    fn create_file(filename: &str, path: &str) -> std::io::Result<()> {
        let full_path = Path::new(path).join(filename);
        if !full_path.exists() {
            let mut file: File = File::create(full_path)?;
            file.write_all(b"{}")?;
        }
        Ok(())
    }

    fn write_to_json<T: Serialize>(path: &str, data: &T) -> std::io::Result<()> {
        let json_string: String = to_string_pretty(data).unwrap();
        let mut file: File = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    fn read_from_json<T: for<'de> Deserialize<'de>>(path: &str) -> std::io::Result<T> {
        let mut file: File = File::open(path)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;
        let data: T = serde_json::from_str(&content).unwrap();
        Ok(data)
    }
}
