use super::super::{
    script,
    types::{CommandType, TargetType},
};
use super::{builtins::BUILTIN_COMMANDS, help};
use std::path::PathBuf;

pub fn execute(
    command_directory_path: &str,
    shell: &str,
    args: &[String],
) -> Result<String, String> {
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target = PathBuf::from(&command_directory_path);
    let mut target_type = TargetType::Directory;
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
    while let Some(arg) = args_peekable.peek() {
        target.push(arg);
        if target.is_file() {
            target_type = TargetType::File;
            args_peekable.next();
            break;
        } else if target.is_dir() {
            target_type = TargetType::Directory;
            args_peekable.next();
        } else {
            // the current argument does not match
            // a directory or a file, so we've landed
            // on the strictest match.
            target.pop();
            break;
        }
    }
    let remaining_args: Vec<_> = args_peekable.collect();
    return match target_type {
        TargetType::Directory => match remaining_args.len() {
            0 => Err(format!(
                "{} is a directory. tab-complete to choose subcommands",
                target.to_str().unwrap_or("")
            )),
            _ => Err(format!(
                "command {} not found in directory {}",
                remaining_args[0],
                target.to_str().unwrap_or("")
            )),
        },
        TargetType::File => match script::Script::load(target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(CommandType::Execute, &shell, &remaining_args),
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
}
