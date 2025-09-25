use super::script;
use std::{fs::read_dir, io, path::Path};

/// scan a directory for all files,
/// consuming each one as a script.
/// returns the invocation
pub fn scan_directory(
    root: &str,
    previous_commands: &mut Vec<String>,
) -> io::Result<Vec<(String, script::Script)>> {
    let mut result = vec![];
    let paths: Vec<_> = read_dir(root).unwrap().map(|r| r.unwrap()).collect();
    // paths.sort_by_key(|f| f.path());
    for entry in paths {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
        previous_commands.push(
            file_name.to_string(),
        );
        if path.is_dir() {
            if is_tome_script_directory(&path) {
                result.extend(scan_directory(
                    path.as_path().to_str().unwrap_or_default(),
                    previous_commands,
                )?);
            }
        } else if script::is_tome_script(file_name) {
            result.push((
                previous_commands.join(" "),
                script::Script::load(path.as_path().to_str().unwrap_or_default())?,
            ));
        }
        previous_commands.pop();
    }
    Ok(result)
}

/// returns if this directory should be considered by tome
pub fn is_tome_script_directory(dir: &Path) -> bool {
    let mut tomeignore_location = dir.to_path_buf();
    // ignore dot directories
    if tomeignore_location
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .starts_with('.')
    {
        return false;
    }
    tomeignore_location.push(".tomeignore");
    !tomeignore_location.exists()
}
