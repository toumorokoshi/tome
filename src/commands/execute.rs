use super::super::{finder, script, types::CommandType};
use super::{builtins::BUILTIN_COMMANDS, help};

pub fn execute(
    command_directory_path: &str,
    shell: &str,
    args: &[String],
) -> Result<String, String> {
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut args_peekable = args.iter().peekable();
    // if no argument is passed, return help.
    if args_peekable.peek().is_none() {
        return help::help(command_directory_path);
    }
    // special handling for root subcommmand for reserved
    // commands
    match args_peekable.peek() {
        Some(&command_name) => match BUILTIN_COMMANDS.get(command_name) {
            Some(command) => return (command.func)(command_directory_path, shell, args),
            None => {}
        },
        None => {}
    }
    // generic handling
    match finder::find_script(command_directory_path, args) {
        Ok(script_invocation) => {
            match script::Script::load(script_invocation.target.to_str().unwrap_or_default()) {
                Ok(script) => {
                    script.get_execution_body(CommandType::Execute, &shell, &script_invocation.args)
                }
                Err(error) => return Err(format!("IOError loading file: {:?}", error)),
            }
        }
        Err(err) => Err(err),
    }
}
