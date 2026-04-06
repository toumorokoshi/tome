use super::super::{finder, finder::FindResult, script, types::CommandType};
use super::{builtins::BUILTIN_COMMANDS, help};

pub fn execute(
    command_directory_path: &str,
    shell: &str,
    args: &[String],
) -> Result<String, String> {
    let mut args_peekable = args.iter().peekable();
    if args_peekable.peek().is_none() {
        return help::help(command_directory_path, &[]);
    }
    if let Some(&command_name) = args_peekable.peek() {
        if let Some(command) = BUILTIN_COMMANDS.get(command_name) {
            return (command.func)(command_directory_path, shell, args);
        }
    }
    match finder::find_script(command_directory_path, args) {
        Ok(FindResult::File(invocation)) => {
            match script::Script::load(invocation.target.to_str().unwrap_or_default()) {
                Ok(script) => {
                    script.get_execution_body(CommandType::Execute, shell, &invocation.args)
                }
                Err(error) => Err(format!("IOError loading file: {:?}", error)),
            }
        }
        Ok(FindResult::Directory(path)) => Err(format!(
            "{} is a directory. tab-complete to choose subcommands",
            path.to_str().unwrap_or("")
        )),
        Err(err) => Err(err),
    }
}

