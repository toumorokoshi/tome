use super::super::directory::scan_directory;
use super::super::finder;
use super::super::script::Script;
use super::builtins::BUILTIN_COMMANDS;
use std::path::PathBuf;

macro_rules! help_template {
    () => {
        r#"echo -e
'This is an instance of tome, running against the directory {}.
\nThe commands are namespaced by the directory structure.
\nBuiltin commands available to all instance of tome are:
\n    {}
\n{}commands available are:
\n    {}
';"#
    };
}

pub fn help(root: &str, args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return help_all(root, "Full list of ");
    }
    let invocation = match finder::find_script(root, args) {
        Ok(inv) => inv,
        Err(_) => return help_dir_scoped(root, args),
    };
    let script = Script::load(invocation.target.to_str().unwrap_or_default())
        .map_err(|e| format!("IOError loading file: {:?}", e))?;
    Ok(help_for_script(&invocation.target, &script))
}

fn help_for_script(path: &PathBuf, script: &Script) -> String {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let summary = escape_slashes(&script.summary_string);
    let help = escape_slashes(&script.help_string);
    format!(
        "echo -e\n'{}: {}\n{}';\n",
        escape_slashes(name),
        summary,
        help
    )
}

fn help_dir_scoped(root: &str, args: &[String]) -> Result<String, String> {
    let mut target = PathBuf::from(root);
    for arg in args {
        target.push(arg);
    }
    if !target.is_dir() {
        return Err(format!(
            "command {} not found",
            args.last().unwrap_or(&String::new())
        ));
    }
    let dir_str = target.to_str().unwrap_or_default();
    let scope_label = args.join(" ");
    help_all_in_dir(root, dir_str, &format!("The list of {} ", scope_label))
}

fn help_all(root: &str, label: &str) -> Result<String, String> {
    help_all_in_dir(root, root, label)
}

fn help_all_in_dir(root: &str, dir: &str, list_label: &str) -> Result<String, String> {
    let mut builtins_with_help = vec![];
    for (command, command_struct) in BUILTIN_COMMANDS.iter() {
        builtins_with_help.push(format!(
            "    {}: {}",
            escape_slashes(command),
            escape_slashes(command_struct.help_text),
        ))
    }

    let commands_and_scripts = match scan_directory(dir, &mut vec![]) {
        Ok(result) => result,
        Err(io_error) => return Err(format!("{}", io_error)),
    };
    let commands_with_help: Vec<_> = commands_and_scripts
        .iter()
        .map(|(command, script)| {
            format!(
                "    {}: {}",
                escape_slashes(command),
                escape_slashes(&script.summary_string)
            )
        })
        .collect();

    Ok(format!(
        help_template!(),
        root,
        builtins_with_help.join("\\n"),
        list_label,
        commands_with_help.join("\\n"),
    ))
}

fn escape_slashes(s: &str) -> String {
    s.replace('\'', "'\\''")
}
