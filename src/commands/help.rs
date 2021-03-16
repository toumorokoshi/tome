use super::super::directory::scan_directory;
use std::{io, iter::Peekable, slice::Iter};

macro_rules! help_template {
    () => {
        r#"
This is an instance of tome, running against the directory {}.
The commands are namespaced by the directory structure.
Full list of commands available are:
    {}
"#;
    };
}

pub fn help(root: &str, mut _args: Peekable<Iter<String>>) -> io::Result<String> {
    let mut commands_with_help = vec![];
    // TODO: restrict scan directory to reject hidden files starting with `.`
    for (command, script) in scan_directory(root, &mut vec![])? {
        commands_with_help.push(format!(
            "    {}: {}",
            &command,
            &script.summary_string
        ))
    }
    Ok(format!(
        help_template!(),
        root,
        commands_with_help.join("\n")
    ))
}