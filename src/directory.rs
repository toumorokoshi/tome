use super::script;
use std::{fs, io, path::Path};

pub struct TomeIgnorer {
    gitignore: ignore::gitignore::Gitignore,
}

impl TomeIgnorer {
    pub fn new(root: &Path) -> Self {
        let mut builder = ignore::gitignore::GitignoreBuilder::new(root);
        let tomeignore_path = root.join(".tomeignore");
        if tomeignore_path.exists() {
            if let Ok(metadata) = fs::metadata(&tomeignore_path) {
                if metadata.len() > 0 {
                    builder.add(tomeignore_path);
                }
            }
        }
        let gitignore = builder.build().unwrap_or(ignore::gitignore::Gitignore::empty());
        Self { gitignore }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        // ignore dot directories and files
        if path.file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .starts_with('.') {
            return true;
        }

        // old behavior fallback: if there's an empty .tomeignore in a directory, ignore that directory
        if path.is_dir() {
            let tomeignore_path = path.join(".tomeignore");
            if tomeignore_path.exists() {
                if let Ok(metadata) = fs::metadata(&tomeignore_path) {
                    if metadata.len() == 0 {
                        return true;
                    }
                }
            }
        }

        self.gitignore.matched_path_or_any_parents(path, path.is_dir()).is_ignore()
    }
}

/// scan a directory for all files,
/// consuming each one as a script.
pub fn scan_directory(
    root: &str,
    previous_commands: &mut Vec<String>,
    ignorer: &TomeIgnorer,
) -> io::Result<Vec<(String, script::Script)>> {
    let mut result = vec![];
    let paths_raw = fs::read_dir(root);
    if paths_raw.is_err() {
        return Ok(vec![]);
    }
    let paths: Vec<_> = paths_raw.unwrap().map(|r| r.unwrap()).collect();
    for entry in paths {
        let path = entry.path();
        
        if ignorer.is_ignored(&path) {
            continue;
        }

        let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
        let command_name = script::strip_source_suffix(file_name).to_string();
        previous_commands.push(command_name);
        if path.is_dir() {
            result.extend(scan_directory(
                path.as_path().to_str().unwrap_or_default(),
                previous_commands,
                ignorer,
            )?);
        } else if script::is_tome_script(&path) {
            result.push((
                previous_commands.join(" "),
                script::Script::load(path.as_path().to_str().unwrap_or_default())?,
            ));
        }
        previous_commands.pop();
    }
    Ok(result)
}

