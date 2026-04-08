use std::{
    fs::File,
    io,
    io::{prelude::*, BufReader, Read},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Stdio},
};

use super::types::CommandType;

const SOURCE_EXTENSION: &str = "source";

// used to determine if the file is a valid script or not
pub fn is_tome_script(path: &Path) -> bool {
    let filename = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    if filename.starts_with('.') {
        return false;
    }
    if is_source_file(filename) {
        return true;
    }
    is_executable(path)
}

pub fn is_source_file(filename: &str) -> bool {
    Path::new(filename)
        .extension()
        .map_or(false, |ext| ext == SOURCE_EXTENSION)
}

fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn strip_source_suffix(name: &str) -> &str {
    name.strip_suffix(".source").unwrap_or(name)
}

/// Try to resolve a path that may have a `.source` extension on disk.
/// Returns the resolved path if found, or None.
pub fn resolve_source_path(path: &Path) -> Option<std::path::PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }
    let source_path = path.with_extension(SOURCE_EXTENSION);
    if source_path.is_file() {
        return Some(source_path);
    }
    None
}

/// Any executable script
/// can be added to be executed, but
/// It's possible to add metadata
/// to the script via comments as well.
pub struct Script {
    pub help_string: String,
    /// the string that should be used for
    /// usage information
    /// the path the script is located at.
    pub path: String,
    /// determines if the script should
    /// be sourced or not.
    pub should_source: bool,
    /// determines if the script should
    /// have completion invoked or not.
    pub should_complete: bool,
    /// the string that should be printed
    /// when help is requested.
    pub summary_string: String,
}

impl Script {
    pub fn load(path: &str) -> io::Result<Script> {
        let file = Box::new(File::open(path)?) as Box<dyn Read>;
        Ok(Script::load_from_buffer(path.to_owned(), file))
    }
    pub fn load_from_buffer(path: String, body: Box<dyn Read>) -> Script {
        let mut buffer = BufReader::new(body);
        let mut should_complete = false;
        let should_source = is_source_file(&path);
        let mut help_string = String::new();
        let mut summary_string = String::new();
        let mut line = String::new();
        let mut consuming_help = false;
        loop {
            line.clear();
            match buffer.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                }
                Err(_) => break,
            }
            if consuming_help {
                if line.starts_with("# END HELP") {
                    consuming_help = false;
                } else if let Some(rest) = line.strip_prefix("# ") {
                    // omit first two characters since they are
                    // signifying continued help.
                    help_string.push_str(rest);
                }
            } else if line.starts_with("# COMPLETE") {
                should_complete = true;
            } else if line.starts_with("# START HELP") {
                consuming_help = true;
            } else if line.starts_with("# SUMMARY: ") {
                // 11 = prefix, -1 strips newline
                summary_string.push_str(&line[11..(line.len() - 1)]);
            } else if !line.starts_with("#!") {
                // if a shebang is encountered, we skip.
                // as it can indicate the command to run the script with.
                // metadata lines must be consecutive.
                break;
            }
        }
        Script {
            help_string,
            path,
            should_complete,
            should_source,
            summary_string,
        }
    }

    // return the appropriate string that should be executed within the
    // function.
    pub fn get_execution_body(
        &self,
        command_type: CommandType,
        shell: &str,
        args: &[&String],
    ) -> Result<String, String> {
        match command_type {
            CommandType::Completion => {
                if !self.should_complete {
                    return Ok(String::new());
                }
                // in the completion case, we need to execute the script itself.
                // There's a possible optimization here
                // if we just inherit parent file descriptors.
                let mut command = match self.should_source {
                    true => Command::new(shell),
                    false => Command::new(self.path.clone()),
                };
                if self.should_source {
                    command.arg(self.path.clone());
                }
                command.arg("--complete");
                let command_output = command.args(args).stdout(Stdio::piped()).output();
                match command_output {
                    Ok(output) => match String::from_utf8(output.stdout) {
                        Err(error) => Err(format!(
                            "unable to parse completion results as a utf8 string: {}",
                            error
                        )),
                        Ok(result) => Ok(result),
                    },
                    // TODO: it's hard to get output from a completion call.
                    // possible to print to stderr?
                    Err(result) => Err(format!("completion called failed: {}", result)),
                }
            }
            CommandType::Execute => {
                let command_string = if self.should_source {
                    // when sourcing, just return the full body.
                    let mut command = vec![String::from("."), self.path.clone()];
                    for arg in args.iter() {
                        command.push((**arg).clone());
                    }
                    command
                } else {
                    let mut command = vec![self.path.clone()];
                    for arg in args.iter() {
                        command.push((**arg).clone());
                    }
                    command
                };
                // after figuring out the command, all resolved values
                // should be quoted, to ensure that the shell does not
                // interpret character sequences.
                let mut escaped_command_string = vec![];
                for mut arg in command_string {
                    arg = arg.replace('\'', "\\'");
                    arg.insert(0, '\'');
                    arg.push('\'');
                    escaped_command_string.push(arg);
                }
                Ok(escaped_command_string.join(" "))
            }
        }
    }
}
