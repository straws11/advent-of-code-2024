use std::{fs, path::Path};
pub fn read_file(file_path: &Path) -> Result<Vec<String>, String> {
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => return Err(format!("Cannot read file at {:?}", file_path).to_string()),
    };

    let mut lines = contents
        .split("\n")
        // .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop();
        }
    }
    Ok(lines)
}
