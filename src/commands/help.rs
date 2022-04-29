use super::super::directory::scan_directory;
use super::builtins::BUILTIN_COMMANDS;

macro_rules! help_template {
    () => {
        r#"echo -e
'This is an instance of tome, running against the directory {}.
\nThe commands are namespaced by the directory structure.
\nBuiltin commands available to all instance of tome are:
\n    {}
\nFull list of commands available are:
\n    {}
';"#
    };
}

pub fn help(root: &str) -> Result<String, String> {
    let mut builtins_with_help = vec![];
    // print builtins first
    for (command, command_struct) in BUILTIN_COMMANDS.iter() {
        builtins_with_help.push(format!(
            "    {}: {}",
            escape_slashes(&command),
            escape_slashes(&command_struct.help_text),
        ))
    }

    let mut commands_with_help = vec![];
    let commands_and_scripts = match scan_directory(root, &mut vec![]) {
        Ok(result) => result,
        Err(io_error) => return Err(format!("{}", io_error)),
    };
    for (command, script) in commands_and_scripts {
        commands_with_help.push(format!(
            "    {}: {}",
            escape_slashes(&command),
            escape_slashes(&script.summary_string)
        ))
    }

    Ok(format!(
        help_template!(),
        root,
        builtins_with_help.join("\\n"),
        commands_with_help.join("\\n"),
    ))
}

// escape slash characters with posix-compatible quotes. Helps if the echo
// command uses slashes
fn escape_slashes(s: &str) -> String {
    s.replace('\'', "'\\''")
}
