use anyhow::Result;
use std::{
    fs::{write, File},
    io::stdin,
    path::{Path, PathBuf},
};

pub fn save_str_in_file(save_path: String, content: String) -> Result<()> {
    write(save_path, content)?;
    Ok(())
}

pub fn verify_file(s: &str) -> Result<String, String> {
    if Path::new(s).exists() || s == "-" {
        Ok(s.into())
    } else {
        Err(format!("指定文件不存在: {}", s))
    }
}

pub fn verify_dir(s: &str) -> Result<PathBuf, String> {
    let path = Path::new(s);
    if path.exists() && path.is_dir() {
        Ok(s.into())
    } else {
        Err(format!("指定目录不存在: {}", s))
    }
}

pub fn get_reader_from_path(path: &str) -> Result<Box<dyn std::io::Read>> {
    if path == "-" {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(path)?))
    }
}

pub fn get_string_from_path(path: &str) -> Result<String> {
    let mut buffer = String::new();
    let mut reader = get_reader_from_path(path)?;

    reader.read_to_string(&mut buffer)?;

    Ok(buffer)
}
