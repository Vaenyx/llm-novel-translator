use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn remove_path(path_str: &str) -> Result<()> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Ok(());
    }

    let metadata = fs::symlink_metadata(path)?;

    if metadata.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }

    return Ok(());
}

pub fn save_file(content: &str, out: &str) -> Result<()> {
    std::fs::write(out, content)?;
    return Ok(());
}
