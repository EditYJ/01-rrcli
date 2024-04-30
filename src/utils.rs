use anyhow::Result;
use std::fs::write;

pub fn save_str_in_file(save_path: String, content: String) -> Result<()> {
    write(save_path, content)?;
    Ok(())
}
