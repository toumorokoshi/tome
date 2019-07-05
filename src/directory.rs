use super::commands::Script;
use std::{fs::read_dir, io};

/// scan a directory for all files,
/// consuming each one as a script.
/// returns the invocation
pub fn scan_directory(
    root: &str,
    previous_commands: &mut Vec<String>,
) -> io::Result<Vec<(String, Script)>> {
    let mut result = vec![];
    for entry in read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        previous_commands.push(
            path.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string(),
        );
        if path.is_dir() {
            result.extend(scan_directory(
                &path.as_path().to_str().unwrap_or_default(),
                previous_commands,
            )?);
        } else {
            result.push((
                previous_commands.join(" "),
                Script::load(&path.as_path().to_str().unwrap_or_default())?,
            ));
        }
        previous_commands.pop();
    }
    Ok(result)
}
