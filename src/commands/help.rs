use super::super::directory::scan_directory;
use std::{io, iter::Peekable, slice::Iter};

macro_rules! help_template {
    () => {
        r#"echo -e
'This is an instance of tome, running against the directory {}.
\nThe commands are namespaced by the directory structure.
\nFull list of commands available are:
\n    {}
';"#;
    };
}

pub fn help(root: &str, mut _args: Peekable<Iter<String>>) -> io::Result<String> {
    let mut commands_with_help = vec![];
    for (command, script) in scan_directory(root, &mut vec![])? {
        commands_with_help.push(format!(
            "    {}: {}",
            escape_slashes(&command),
            escape_slashes(&script.summary_string)
        ))
    }
    Ok(format!(
        help_template!(),
        root,
        commands_with_help.join("\\n")
    ))
}

// escape slash characters with posix-compatible quotes. Helps if the echo
// command uses slashes
fn escape_slashes(s: &str) -> String {
    s.replace("'", "'\\''")
}
