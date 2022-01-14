use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ShellType {
    BASH,
    FISH,
    UNKNOWN,
    ZSH,
}

/// Determine the shell type from the shell_executable_path
/// passed.
///
/// If no string is passed, attempt to infer the information from the
/// "SHELL" environment variable.
///
/// This function is designed to infer the shell from a
/// variety of architectures and operating systems.
pub fn get_shell_type(shell_executable_path: &str) -> Result<ShellType, String> {
    // strip any leading directories. The root should be the shell.
    let shell_type = match Path::new(shell_executable_path).file_stem() {
        Some(p) => match p.to_str() {
            Some(inner_p) => inner_p,
            None => {
                return Err(format!(
                    "could not covert shell type to string for '{}'",
                    shell_executable_path
                ));
            }
        },
        None => {
            return Err(format!(
                "could not extract file stem to determine shell type from '{}'",
                shell_executable_path
            ));
        }
    };

    if shell_type.ends_with("fish") {
        return Ok(ShellType::FISH);
    }

    if shell_type.ends_with("bash") {
        return Ok(ShellType::BASH);
    }

    if shell_type.ends_with("zsh") {
        return Ok(ShellType::ZSH);
    }

    return Ok(ShellType::UNKNOWN);
}
