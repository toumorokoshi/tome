use super::super::directory::scan_directory;
use super::super::finder::{self, FindResult};
use super::super::script::Script;
use super::builtins::BUILTIN_COMMANDS;
use std::path::PathBuf;

macro_rules! help_template {
    () => {
        r#"echo -e
'This is an instance of tome, running against the directory {}.
\nThe commands are namespaced by the directory structure.
\n
\n{}commands available are:
\n    {}
\n
\nBuiltin commands available to all instance of tome are:
\n    {}
';"#
    };
}

pub fn help(root: &str, args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return help_all(root, "Full list of ");
    }
    match finder::find_script(root, args) {
        Ok(FindResult::File(invocation)) => {
            let script = Script::load(invocation.target.to_str().unwrap_or_default())
                .map_err(|e| echo_error(&format!("IOError loading file: {:?}", e)))?;
            Ok(help_for_script(&invocation.target, &script))
        }
        Ok(FindResult::Directory(path)) => {
            let dir_str = path.to_str().unwrap_or_default();
            let scope_label = args.join(" ");
            help_all_in_dir(root, dir_str, &format!("The list of {} ", scope_label))
        }
        Err(err) => Ok(echo_error(&err)),
    }
}

fn echo_error(msg: &str) -> String {
    format!("echo '{}' >&2", escape_slashes(msg))
}

fn help_for_script(path: &PathBuf, script: &Script) -> String {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let summary = escape_slashes(&script.summary_string);
    let help = escape_slashes(&script.help_string);
    format!(
        "echo -e\n'{}: {}\n{}';",
        escape_slashes(name),
        summary,
        help
    )
}

fn help_all(root: &str, label: &str) -> Result<String, String> {
    help_all_in_dir(root, root, label)
}

fn help_all_in_dir(root: &str, dir: &str, list_label: &str) -> Result<String, String> {
    let builtins_with_help: Vec<_> = BUILTIN_COMMANDS
        .iter()
        .map(|(command, command_struct)| {
            format!(
                "    {}: {}",
                escape_slashes(command),
                escape_slashes(command_struct.help_text),
            )
        })
        .collect();

    let ignorer = super::super::directory::TomeIgnorer::new(std::path::Path::new(root));
    let commands_and_scripts =
        scan_directory(dir, &mut vec![], &ignorer).map_err(|e| echo_error(&format!("{}", e)))?;
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
        commands_with_help.join("\\n"),
        list_label,
        builtins_with_help.join("\\n"),
    ))
}

fn escape_slashes(s: &str) -> String {
    s.replace('\'', "'\\''")
}
